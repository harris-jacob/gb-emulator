#include "cpu.h"
#include "register.h"
#include "mmu.h"


/* OP00 - NOP */
static void OP_00(cpu_t* cpu) {
    return;
}

/* OP01 - ld bc d16 */
static void OP_01(cpu_t* cpu, uint16_t val) {
    cpu->reg->bc = val;
}

/* OP02 - ld (bc),a */
static void OP_02(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->bc, cpu->reg->a);
}

/* OP03 - INC BC */
static void OP_03(cpu_t* cpu) {
    cpu->reg->bc++;
}

/* OP04 - INC B */
static void OP_04(cpu_t* cpu) {
    cpu->reg->b = alu_inc8(cpu->reg, cpu->reg->b);
}

/* OP05 - DECB */
static void OP_05(cpu_t* cpu) {
    cpu->reg->b = alu_dec8(cpu->reg, cpu->reg->b);
}


/* OP06 - LD, B, d8 */
static void OP_06(cpu_t* cpu, uint8_t val) {
    cpu->reg->b = val;
}

/* OP07 - RLCA */
static void OP_07(cpu_t* cpu, uint8_t val) {
   cpu->reg->a = rl(cpu->reg, cpu->reg->a);
}

/* OP08 - LD (a16), SP  */
static void OP_08(cpu_t* cpu, uint16_t addr) {
    mmu_write_addr16(cpu->mmu, addr, cpu->reg->sp);
}


/* OP09 - ADD HL, BC */
static void OP_09(cpu_t* cpu) {
    cpu->reg->hl++;
}

/* OP0A - LD A,(BC) */
static void OP_0A(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->bc);
    cpu->reg->a = val;
}

/* OP0B - DEC BC */
static void OP_0B(cpu_t* cpu) {
   cpu->reg->bc--;
}

/* OP0C - INC C */
static void OP_0C(cpu_t* cpu) {
    cpu->reg->c = alu_inc8(cpu->reg, cpu->reg->c);
}

/* OP0D - DEC C */
static void OP_0D(cpu_t* cpu) {
    cpu->reg->c = alu_dec8(cpu->reg, cpu->reg->c);
}

/* OP0E - LD, C, d8 */
static void OP_0E(cpu_t* cpu, uint8_t val) {
    cpu->reg->c = val;
}

/* OP0F - RRCA */
static void OP_0F(cpu_t* cpu, uint8_t val) {
   cpu->reg->a = rr(cpu->reg, cpu->reg->a);
}

/* OP10 - STOP d8 */
static void OP_10(cpu_t* cpu, uint8_t val) {
    cpu->stopped = false;
}

/* OP11 - LD DE d16 */
static void OP_11(cpu_t* cpu, uint16_t val) {
    cpu->reg->de = val;
} 

/* OP12- LD (DE) A */
static void OP_12(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->de, cpu->reg->a);
}

/* OP13 - INC DE */   
static void OP_13(cpu_t* cpu) {
    cpu->reg->de++;
}

/* OP14 - INC D */
static void OP_14(cpu_t* cpu) {
    cpu->reg->d = alu_inc8(cpu->reg, cpu->reg->d);

}

/* OP15 - DEC D */
static void OP_15(cpu_t* cpu) {
    cpu->reg->d = alu_dec8(cpu->reg, cpu->reg->d);
}

/* OP16 - LD D,d8 */
static void OP_16(cpu_t* cpu, uint8_t val) {
    cpu->reg->d = val;
}

/* OP17 - RLA */
static void OP_17(cpu_t* cpu) {
    
    reset_zero(cpu->reg);
    reset_halfcarry(cpu->reg);
    reset_subtract(cpu->reg);
    uint8_t prev_car = get_carry(cpu->reg);
    if(cpu->reg->a & 1 == 1) {
       set_carry(cpu->reg);
    } else {
       reset_carry(cpu->reg);
    }

    if(prev_car == 0) {
       cpu->reg->a = cpu->reg->a >> 1;
    } else {
        cpu->reg->a = (cpu->reg->a >> 1) | 128;
    }
}

/* OP18 - JR r8 */
static void OP_18(cpu_t* cpu, uint8_t val) {
     short result = cpu->reg->pc + (short)val;
     cpu->reg->pc = result;
}

/* OP19 - ADD HL, DE */
static void OP_19(cpu_t* cpu) {
    cpu->reg->hl = alu_add16(cpu->reg, cpu->reg->hl, cpu->reg->de);
}

/* OP1A - LD A (DE) */
static void OP_1A(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->de);
    cpu->reg->a = val;
}

/* OP1B - DEC DE */
static void OP_1B(cpu_t* cpu) {
    cpu->reg->de--;
}

/* OP1C - INC E */
static void OP_1C(cpu_t* cpu) {
    cpu->reg->e = alu_inc8(cpu->reg, cpu->reg->e);
}

/* OP1D - DEC E */
static void OP_1D(cpu_t* cpu) {
    cpu->reg->e = alu_dec8(cpu->reg, cpu->reg->e);
}

/* OP1E - LD E, d8 */
static void OP_1E(cpu_t* cpu, uint8_t val) {
    cpu->reg->e = val;
}

/* OP1F - RRA */
static void OP_1F(cpu_t* cpu) {
    cpu->reg->a =rr(cpu->reg, cpu->reg->a);
}

/* OP20 - JR NZ r8 */
static void OP_20(cpu_t* cpu, unsigned char val) {
    if(!get_zero(cpu->reg)) {
     cpu->reg->pc += (signed char)val;
     // extra cycles
     cpu->clock_cycle+=4;
    }
}

/* OP21 - LD HL d16 */
static void OP_21(cpu_t* cpu, uint16_t val) {
    cpu->reg->hl = val; 
}

/* OP22 - LD (HL+) A */
static void OP_22(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl++, cpu->reg->a);
}

/* OP23 - INC HL*/
static void OP_23(cpu_t* cpu) {
    cpu->reg->hl++;
}

/* OP24 - INC H */
static void OP_24(cpu_t* cpu) {
    cpu->reg->h = alu_inc8(cpu->reg, cpu->reg->h);
}

/* OP25 - DEC H */
static void OP_25(cpu_t* cpu) {
    cpu->reg->h = alu_dec8(cpu->reg, cpu->reg->h);
}

/* OP26 - LD H d8 */
static void OP_26(cpu_t* cpu, uint8_t val) {
    cpu->reg->h = val;
}

/* OP27 - DAA */
static void OP_27(cpu_t* cpu) {
    uint16_t val = cpu->reg->a;

    if(get_subtract(cpu->reg)) {
        if(get_halfcarry(cpu->reg)) {
          val = (val - 0x06)&0xff;
        }
        if(get_carry(cpu->reg)) {
            val -= 0x60;
        }
    } else {
        if(get_halfcarry(cpu->reg) || (val & 0xF) > 9) {
            val += 0x06;
        }
        if(get_carry(cpu->reg) || val > 0x9f) {
            val += 0x60;
        }
    }

    reset_halfcarry(cpu->reg);

    if(cpu->reg->a) {
        reset_zero(cpu->reg);
    } else {
        set_zero(cpu->reg);
    }

    if(val >= 0x100) {
        set_carry(cpu->reg);
    } else {
        reset_carry(cpu->reg);
    }

    cpu->reg->a = (uint8_t)(val);

}

/* OP28 - JR Z, r8 */
static void OP_28(cpu_t* cpu, uint8_t val) {

    signed char signed_val = (signed char)val;

   if(get_zero(cpu->reg)) {
     cpu->reg->pc += signed_val;
     // extra cycles
     cpu->clock_cycle+=4;
   } 
}

/* OP29 - ADD HL HL */
static void OP_29(cpu_t* cpu) {
    cpu->reg->hl = alu_add16(cpu->reg, cpu->reg->hl, cpu->reg->hl);
}

/* OP2A - LD A (HL+) */
static void OP_2A(cpu_t* cpu) {
    cpu->reg->a = mmu_read_addr8(cpu->mmu, cpu->reg->hl++);
}

/* OP2B - DEC HL */
static void OP_2B(cpu_t* cpu) {
    cpu->reg->hl--;
}

/* OP2C - INC L */
static void OP_2C(cpu_t* cpu) {
    cpu->reg->l = alu_inc8(cpu->reg, cpu->reg->l);
}

/* OP2D - DEC L */
static void OP_2D(cpu_t* cpu) {
    cpu->reg->l = alu_dec8(cpu->reg, cpu->reg->l);
}

/* OP2E - LD L  d8*/
static void OP_2E(cpu_t* cpu, uint8_t val) {
    cpu->reg->l = val; 
}

/* OP2F - CPL */
static void OP_2F(cpu_t* cpu) {
    cpu->reg->a = 0xff ^ cpu->reg->a;

    set_subtract(cpu->reg);
    set_halfcarry(cpu->reg);
}

/* OP30 - JR NC r8 */ 
static void OP_30(cpu_t* cpu, uint8_t val) {
    if(!get_carry(cpu->reg)) {
        cpu->reg->pc += (signed char)val;
        // extra cylces
        cpu->clock_cycle+=4;
    }
}

/* OP31 - LD SP d16*/
static void OP_31(cpu_t* cpu, uint16_t val) {
    cpu->reg->sp = val;
}

/* OP32 - (HL-) a */
static void OP_32(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, cpu->reg->a);
    cpu->reg->hl--;
}

/* OP33 - INC SP */
static void OP_33(cpu_t* cpu) {
    cpu->reg->sp++;
}

/* OP34 - INC (HL) */
static void OP_34(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    uint8_t newVal = alu_inc8(cpu->reg, val);
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, newVal);
}

/* OP35 - DEC (HL) */
static void OP_35(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    uint8_t newVal = alu_dec8(cpu->reg, val);
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, newVal);
}

/* OP36- LD (HL) d8 */
static void OP_36(cpu_t* cpu, uint8_t val) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, val);
}

/* OP37 - SCF */
static void OP_37(cpu_t* cpu, uint8_t val) {
    set_carry(cpu->reg);
    reset_subtract(cpu->reg);
    reset_halfcarry(cpu->reg);
}

/* OP38 - JR C, r8 */
static void OP_38(cpu_t* cpu, uint8_t val) {
    if(get_carry(cpu->reg)) {
        short result = cpu->reg->pc + (signed char)val;
        cpu->reg->pc = result;

        // extra cycles
        cpu->clock_cycle+=4;
    }
}

/* OP39 - ADD HL, SP */
static void OP_39(cpu_t* cpu) {
    cpu->reg->hl = cpu->reg->hl + cpu->reg->sp;
}

/* OP3A - LD A, HL- */
static void OP_3A(cpu_t* cpu) {
    cpu->reg->a = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->hl--;
}

/* OP3B - DEC SP*/
static void OP_3B(cpu_t* cpu) {
    cpu->reg->sp--;
}

/* OP3C - INC A */
static void OP_3C(cpu_t* cpu) {
    cpu->reg->a = alu_inc8(cpu->reg, cpu->reg->a);
}

/* OP3D - DEC A */
static void OP_3D(cpu_t* cpu) {
    cpu->reg->a = alu_dec8(cpu->reg, cpu->reg->a);
}

/* OP3E - LD A, d8 */
static void OP_3E(cpu_t* cpu, uint8_t val) {
    cpu->reg->a = val; 
}

/* OP3F - CCF */
static void OP_3F(cpu_t* cpu, uint8_t val) {
    if(get_carry(cpu->reg)) {
        reset_carry(cpu->reg);
    } else set_carry(cpu->reg);

    // reset
    reset_subtract(cpu->reg);
    reset_halfcarry(cpu->reg);
}

/* OP40 - LD B, B  */
static void OP_40(cpu_t* cpu) {
    cpu->reg->b = cpu->reg->b;
}

/* OP41 - LD B, C  */
static void OP_41(cpu_t* cpu) {
    cpu->reg->b = cpu->reg->c;
}

/* OP42 - LD B, D */
static void OP_42(cpu_t* cpu) {
    cpu->reg->b = cpu->reg->d;
}

/* OP43 - LD B, E */
static void OP_43(cpu_t* cpu) {
    cpu->reg->b = cpu->reg->e;
}

/* OP44 - LD B, H*/
static void OP_44(cpu_t* cpu) {
    cpu->reg->b = cpu->reg->h;
}

/* OP45 - LD B, L*/
static void OP_45(cpu_t* cpu) {
    cpu->reg->b = cpu->reg->l;
}

/* OP46 - LD B, (HL)*/
static void OP_46(cpu_t* cpu) {
    cpu->reg->b = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
}

/* OP47 - LD B, A*/
static void OP_47(cpu_t* cpu) {
    cpu->reg->b = cpu->reg->a;
}

/* OP48 - LD C, B*/
static void OP_48(cpu_t* cpu) {
    cpu->reg->c = cpu->reg->b;
}

/* OP49 - LD C, C*/
static void OP_49(cpu_t* cpu) {
    cpu->reg->c = cpu->reg->c;
}

/* OP4A - LD C, D*/
static void OP_4A(cpu_t* cpu) {
    cpu->reg->c = cpu->reg->d;
}

/* OP4B - LD C, E*/
static void OP_4B(cpu_t* cpu) {
    cpu->reg->c = cpu->reg->e;
}

/* OP4C - LD C, H*/
static void OP_4C(cpu_t* cpu) {
    cpu->reg->c = cpu->reg->h;
}

/* OP4D- LD C, H*/
static void OP_4D(cpu_t* cpu) {
    cpu->reg->c = cpu->reg->l;
}

/* OP4E- LD C, (HL)*/
static void OP_4E(cpu_t* cpu) {
    cpu->reg->c = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
}

/* OP4F- LD C, A */
static void OP_4F(cpu_t* cpu) {
    cpu->reg->c = cpu->reg->a;
}

/* OP50 LD D, B */
static void OP_50(cpu_t* cpu) {
    cpu->reg->d = cpu->reg->b;
}

/* OP51 LD D, C */
static void OP_51(cpu_t* cpu) {
    cpu->reg->d = cpu->reg->c;
}

/* OP52 LD D, D  */
static void OP_52(cpu_t* cpu) {
    cpu->reg->d = cpu->reg->d;
}

/* OP53 LD D, E  */
static void OP_53(cpu_t* cpu) {
    cpu->reg->d = cpu->reg->e;
}

/* OP54 LD D, H  */
static void OP_54(cpu_t* cpu) {
    cpu->reg->d = cpu->reg->d;
}

/* OP55 LD D, L  */
static void OP_55(cpu_t* cpu) {
    cpu->reg->d = cpu->reg->l;
}

/* OP56 LD D, (HL)  */
static void OP_56(cpu_t* cpu) {
    cpu->reg->d = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
}

/* OP57 LD D, A  */
static void OP_57(cpu_t* cpu) {
    cpu->reg->d = cpu->reg->a;
}

/* OP58 LD E, B  */
static void OP_58(cpu_t* cpu) {
    cpu->reg->e = cpu->reg->b;
}

/* OP59 LD E, C  */
static void OP_59(cpu_t* cpu) {
    cpu->reg->e = cpu->reg->c;
}

/* OP5A LD E, D  */
static void OP_5A(cpu_t* cpu) {
    cpu->reg->e = cpu->reg->d;
}

/* OP5B LD E, E  */
static void OP_5B(cpu_t* cpu) {
    cpu->reg->e = cpu->reg->e;
}

/* OP5C LD E, H  */
static void OP_5C(cpu_t* cpu) {
    cpu->reg->e = cpu->reg->h;
}

/* OP5D LD E, L  */
static void OP_5D(cpu_t* cpu) {
    cpu->reg->e = cpu->reg->l;
}

/* OP5E LD E (HL) */
static void OP_5E(cpu_t* cpu) {
    cpu->reg->e = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
}

/* OP5F LD E (HL) */
static void OP_5F(cpu_t* cpu) {
    cpu->reg->e = cpu->reg->a;
}

/* OP60 H B */
static void OP_60(cpu_t* cpu) {
    cpu->reg->h = cpu->reg->b;
}

/* OP61 H C */
static void OP_61(cpu_t* cpu) {
    cpu->reg->h = cpu->reg->c;
}

/* OP62 H D */
static void OP_62(cpu_t* cpu) {
    cpu->reg->h = cpu->reg->d;
}

/* OP63 H E */
static void OP_63(cpu_t* cpu) {
    cpu->reg->h = cpu->reg->e;
}

/* OP64 H H */
static void OP_64(cpu_t* cpu) {
    cpu->reg->h = cpu->reg->h;
}

/* OP65 H L */
static void OP_65(cpu_t* cpu) {
    cpu->reg->h = cpu->reg->l;
}

/* OP66 H (HL) */
static void OP_66(cpu_t* cpu) {
    cpu->reg->h = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
}

/* OP67 H A */
static void OP_67(cpu_t* cpu) {
    cpu->reg->h = cpu->reg->a;
}

/* OP68 L B */
static void OP_68(cpu_t* cpu) {
    cpu->reg->l = cpu->reg->b;
}

/* OP69 L C */
static void OP_69(cpu_t* cpu) {
    cpu->reg->l = cpu->reg->c;
}

/* OP6A L D */
static void OP_6A(cpu_t* cpu) {
    cpu->reg->l = cpu->reg->d;
}

/* OP6B L D */
static void OP_6B(cpu_t* cpu) {
    cpu->reg->l = cpu->reg->e;
}

/* OP6C L H */
static void OP_6C(cpu_t* cpu) {
    cpu->reg->l = cpu->reg->h;
}

/* OP6D L L */
static void OP_6D(cpu_t* cpu) {
    cpu->reg->l = cpu->reg->l;
}

/* OP6E L (HL) */
static void OP_6E(cpu_t* cpu) {
    cpu->reg->l = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
}

/* OP6F L A */
static void OP_6F(cpu_t* cpu) {
    cpu->reg->l = cpu->reg->a;
}

/* OP70 (HL) B */
static void OP_70(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, cpu->reg->b);
}

/* OP71 (HL) C */
static void OP_71(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, cpu->reg->c);
}

/* OP72 (HL) D */
static void OP_72(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, cpu->reg->d);
}

/* OP73 (HL) E*/
static void OP_73(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, cpu->reg->e);
}

/* OP74 (HL) H */
static void OP_74(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, cpu->reg->h);
}

/* OP75 (HL) L */
static void OP_75(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, cpu->reg->l);
}

/* OP76 HALT  */
static void OP_76(cpu_t* cpu) {
    cpu->halted = true;
}

/* OP77 LD (HL) A  */
static void OP_77(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, cpu->reg->a);
}

/* OP78 LD A B  */
static void OP_78(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->b;
}

/* OP79 LD A C  */
static void OP_79(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->c;
}

/* OP7A LD A D  */
static void OP_7A(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->d;
}

/* OP7B LD A E  */
static void OP_7B(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->e;
}

/* OP7C LD A H  */
static void OP_7C(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->h;
}

/* OP7D LD A L  */
static void OP_7D(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->l;
}

/* OP7E LD A (HL)  */
static void OP_7E(cpu_t* cpu) {
    cpu->reg->a =mmu_read_addr8(cpu->mmu, cpu->reg->hl);
}

/* OP7F LD A A   */
static void OP_7F(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->a;
}

/* OP80 ADD A,B */
static void OP_80(cpu_t* cpu) {
    cpu->reg->a = alu_add8(cpu->reg, cpu->reg->a, cpu->reg->b);
}

/* OP81 ADD A, C */ 
static void OP_81(cpu_t* cpu) {
    cpu->reg->a = alu_add8(cpu->reg, cpu->reg->a, cpu->reg->c);
}

/* OP82 ADD A, D */
static void OP_82(cpu_t* cpu) {
    cpu->reg->a = alu_add8(cpu->reg, cpu->reg->a, cpu->reg->d);
}

/* OP83 ADD A, E */
static void OP_83(cpu_t* cpu) {
    cpu->reg->a = alu_add8(cpu->reg, cpu->reg->a, cpu->reg->e);
}

/* OP84 ADD A, H */
static void OP_84(cpu_t* cpu) {
    cpu->reg->h = alu_add8(cpu->reg, cpu->reg->a, cpu->reg->h);
}

/* OP85 ADD A, L */
static void OP_85(cpu_t* cpu) {
    cpu->reg->h = alu_add8(cpu->reg, cpu->reg->a, cpu->reg->l);
}

/* OP86 ADD A, (HL) */
static void OP_86(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl); 
    cpu->reg->h = alu_add8(cpu->reg, cpu->reg->a, val);
}

/* OP87 ADD A, A */
static void OP_87(cpu_t* cpu) {
    cpu->reg->a = alu_add8(cpu->reg, cpu->reg->a, cpu->reg->a);
}

/* OP88 ADC A, B */
static void OP_88(cpu_t* cpu) {
    alu_adc8(cpu->reg, cpu->reg->b);
}

/* OP89 ADC A, C */
static void OP_89(cpu_t* cpu) {
    alu_adc8(cpu->reg, cpu->reg->c);
}

/* OP8A ADC A, D */
static void OP_8A(cpu_t* cpu) {
    alu_adc8(cpu->reg, cpu->reg->d);
}

/* OP8B ADC A, E */
static void OP_8B(cpu_t* cpu) {
  alu_adc8(cpu->reg, cpu->reg->e); 
}

/* OP8C ADC A, E */
static void OP_8C(cpu_t* cpu) {
   alu_adc8(cpu->reg, cpu->reg->e); 
}

/* OP8D ADC A, L */
static void OP_8D(cpu_t* cpu) {
    alu_adc8(cpu->reg, cpu->reg->l);
}

/* OP8E ADC A, H */
static void OP_8E(cpu_t* cpu) {
    alu_adc8(cpu->reg, cpu->reg->h);
}

static void OP_8F(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    alu_adc8(cpu->reg, val);
}

/* OP90 SUB B */
static void OP_90(cpu_t* cpu) {
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, cpu->reg->b);
}

/* OP91 SUB C */
static void OP_91(cpu_t* cpu) {
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, cpu->reg->c);
}

/* OP92 SUB D */
static void OP_92(cpu_t* cpu) {
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, cpu->reg->d);
}

/* OP93 SUB E */
static void OP_93(cpu_t* cpu) {
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, cpu->reg->e);
}

/* OP94 SUB H */
static void OP_94(cpu_t* cpu) {
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, cpu->reg->h);
}

/* OP95 SUB H */
static void OP_95(cpu_t* cpu) {
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, cpu->reg->l);
}

/* OP95 SUB (HL) */
static void OP_96(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, val);
}

/* OP96 SUB A */
static void OP_97(cpu_t* cpu) {
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, cpu->reg->a);
}

/* OP98 SBC A, B */
static void OP_98(cpu_t* cpu) {
    cpu->reg->a - alu_sbc8(cpu->reg, cpu->reg->a, cpu->reg->b);
}

/* OP99 SBC A, C */
static void OP_99(cpu_t* cpu) {
    cpu->reg->a = alu_sbc8(cpu->reg, cpu->reg->a, cpu->reg->c);
}

/* OP9A SBC A, D */ 
static void OP_9A(cpu_t* cpu) {
    cpu->reg->a = alu_sbc8(cpu->reg, cpu->reg->a, cpu->reg->d);
}

/* OP9B SBC A, E */
static void OP_9B(cpu_t* cpu) {
    cpu->reg->a = alu_sbc8(cpu->reg, cpu->reg->a, cpu->reg->e);
}

/* OP9C SBC A, H */
static void OP_9C(cpu_t* cpu) {
    cpu->reg->a = alu_sbc8(cpu->reg, cpu->reg->a , cpu->reg->h);
}

/* OP9D SBC A, L */
static void OP_9D(cpu_t* cpu) {
    cpu->reg->a = alu_sbc8(cpu->reg, cpu->reg->a, cpu->reg->l);
}

/* OP9E SBC A, (HL) */
static void OP_9E(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->a = alu_sbc8(cpu->reg, cpu->reg->a, val);
}

/* OP9F SBC A, A */
static void OP_9F(cpu_t* cpu) {
    cpu->reg->a = alu_sbc8(cpu->reg, cpu->reg->a, cpu->reg->a);
}

/* OPA0 AND B */
static void OP_A0(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->a & cpu->reg->b;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA1 AND C */
static void OP_A1(cpu_t* cpu) {

    cpu->reg->a = cpu->reg->a & cpu->reg->c;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA2 AND D */
static void OP_A2(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a & cpu->reg->d;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA3 AND E */
static void OP_A3(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a & cpu->reg->e;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA4 AND H */
static void OP_A4(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a & cpu->reg->h;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA5 AND L */
static void OP_A5(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a & cpu->reg->l;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
    reset_zero(cpu->reg);
    }

    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA6 AND (HL) */
static void OP_A6(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->a = cpu->reg->a & val;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }
    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA7 AND A */
static void OP_A7(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a & cpu->reg->a;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA8 XOR B */
static void OP_A8(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a ^ cpu->reg->b;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPA9 XOR C */
static void OP_A9(cpu_t* cpu) {
    cpu->reg->a = cpu->reg->a ^ cpu->reg->c;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
    
}

/* OPAA XOR D */
static void OP_AA(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a ^ cpu->reg->d;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPAB XOR E */
static void OP_AB(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a ^ cpu->reg->e;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPAC XOR H */
static void OP_AC(cpu_t* cpu) {

    cpu->reg->a = cpu->reg->a ^ cpu->reg->h;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPAD XOR L */
static void OP_AD(cpu_t* cpu) {

    cpu->reg->a = cpu->reg->a ^ cpu->reg->l;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPAE XOR (HL) */
static void OP_AE(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->a = cpu->reg->a ^ val;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPAF XOR A */
static void OP_AF(cpu_t* cpu) {

    cpu->reg->a = cpu->reg->a ^ cpu->reg->a;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPB0 OR B */
static void OP_B0(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a | cpu->reg->b;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}



/* OPB1 OR C */
static void OP_B1(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a | cpu->reg->c;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPB2 OR D */
static void OP_B2(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a | cpu->reg->d;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPB3 OR E */
static void OP_B3(cpu_t* cpu) {

    cpu->reg->a = cpu->reg->a | cpu->reg->e;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPB4 OR H */
static void OP_B4(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a | cpu->reg->h;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPB5 OR L */
static void OP_B5(cpu_t* cpu) {
    
    cpu->reg->a = cpu->reg->a | cpu->reg->l;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPB6 OR (HL) */
static void OP_B6(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->a = cpu->reg->a | val;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else  {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPB7 OR A  */
static void OP_B7(cpu_t* cpu) {

    cpu->reg->a |= cpu->reg->a;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}


/* OPB8 CP B */
static void OP_B8(cpu_t* cpu) {
    cp(cpu->reg, cpu->reg->b);
}

/* OPB9 CP C */
static void OP_B9(cpu_t* cpu) {
    cp(cpu->reg, cpu->reg->c);
}

/* OPBA CP D */
static void OP_BA(cpu_t* cpu) {
    cp(cpu->reg, cpu->reg->d);
}

/* OPBB CP E */
static void OP_BB(cpu_t* cpu) {
    cp(cpu->reg, cpu->reg->e);
}

/* OPBC CP H */
static void OP_BC(cpu_t* cpu) {
    cp(cpu->reg, cpu->reg->h);
}

/* OPBD CP L */
static void OP_BD(cpu_t* cpu) {
    cp(cpu->reg, cpu->reg->l);
}

/* OPBE CP (HL) */
static void OP_BE(cpu_t* cpu) {
    uint8_t val = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cp(cpu->reg, val);
}

/* OPBF CP A */
static void OP_BF(cpu_t* cpu) {
    cp(cpu->reg, cpu->reg->a);
}

/* OPC0 RET NZ */
static void OP_C0(cpu_t* cpu) {
    if(get_zero(cpu->reg) != 0 ) {
        cpu->reg->pc = stack_pop(cpu);
        // extra cycles
        cpu->clock_cycle+=12;
    }
}

/* OPC1 POP BC */
static void OP_C1(cpu_t* cpu) {
    cpu->reg->bc = stack_pop(cpu);
}

/* OPC2 JP NZ a16 */
static void OP_C2(cpu_t* cpu, uint16_t val) {
    if(!get_zero(cpu->reg)) {
        cpu->reg->pc = val; 
        // extra cycles
        cpu->clock_cycle+=4;
    }
}

/* OPC3 JP a16 */
static void OP_C3(cpu_t* cpu, uint16_t val) {
    cpu->reg->pc = val;
}


/* OPC4 CALL NZ, a16 */
static void OP_C4(cpu_t* cpu, uint16_t addr) {
    if(!get_zero(cpu->reg)) {
        stack_push(cpu, cpu->reg->pc);
        cpu->reg->pc = addr;
        //extra cycles
        cpu->clock_cycle+=12;
    }
}


/* OPC5 PUSH BC */
static void OP_C5(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->bc);
}

/* OPC6 ADD A, d8*/
static void OP_C6(cpu_t* cpu, uint8_t val) {
    cpu->reg->a = alu_add8(cpu->reg, cpu->reg->a, val);
}

/* OPC7 RST 00H */
static void OP_C7(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = 0;
}

/* OPC8 RET Z */
static void OP_C8(cpu_t* cpu) {
   if(get_zero(cpu->reg)) {
       cpu->reg->pc = stack_pop(cpu);
       // extra cycles
       cpu->clock_cycle+=12;
   } 
}

/* OPC9 RET */
static void OP_C9(cpu_t* cpu) {
    cpu->reg->pc = stack_pop(cpu);
}

/* OPCA JP Z a16 */
static void OP_CA(cpu_t* cpu, uint16_t addr) {
    if(get_zero(cpu->reg)) {
        cpu->reg->pc = addr;
        // extra cycles
        cpu->clock_cycle+=4;
    }
}


/* OPCB PREFIX CB */
static void OP_CB(cpu_t* cpu, uint8_t instruction) {
    extended_ops[instruction].execute(cpu);

    cpu->clock_cycle += extended_ops[instruction].ticks;
}

/* OPCC CALL Z a16 */
static void OP_CC(cpu_t* cpu, uint16_t addr) {
    if(get_zero(cpu->reg)) {
        stack_push(cpu, cpu->reg->pc);
        cpu->reg->pc = addr;
        // extra cycles
        cpu->clock_cycle+=12;
    }
}

/* OPCD CALL a16 */
static void OP_CD(cpu_t* cpu, uint16_t addr) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = addr;
}

/* OPCE ADC A, d8 */
static void OP_CE(cpu_t* cpu, uint8_t val) {
    alu_adc8(cpu->reg, val);
}

/* OPCF RST 08H */
static void OP_CF(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = 0x8;
}

/* OPD0 RET NC */
static void OP_D0(cpu_t* cpu) {
    if(!get_carry(cpu->reg)) {
        cpu->reg->pc = stack_pop(cpu);
        // extra cycles
        cpu->clock_cycle+=12;
    }
}

/* OPD1 POP DE */
static void OP_D1(cpu_t* cpu) {
    cpu->reg->de = stack_pop(cpu);
}

/* OPD2 JP NC a16 */
static void OP_D2(cpu_t* cpu, uint16_t addr) {
    if(!get_carry(cpu->reg)) {
        cpu->reg->pc = addr;
        // extra cycles
        cpu->clock_cycle+=4;
    }
}

/* OPD4 CALL NC a16 */
static void OP_D4(cpu_t* cpu, uint16_t addr) {

    if(!get_carry(cpu->reg)) {
        stack_push(cpu, cpu->reg->pc);
        cpu->reg->pc = addr;
        // extra cycles
        cpu->clock_cycle+=12;
    }
}

/* OPD5 PUSH DE */
static void OP_D5(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->de);
}

/* OPD6 SUB d8 */
static void OP_D6(cpu_t* cpu, uint16_t val) {
    cpu->reg->a = alu_subtract8(cpu->reg, cpu->reg->a, val);
}

/* OPD7 RST 10H */
static void OP_D7(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = 0x10;
}

/* OPD8 RET C */
static void OP_D8(cpu_t* cpu) {
    if(get_carry(cpu->reg)) {
        cpu->reg->pc = stack_pop(cpu);
        // extra cycles
        cpu->clock_cycle+=12;
    }
}

/* OPD9 RETI */
static void OP_D9(cpu_t* cpu) {
    cpu->reg->pc = stack_pop(cpu);
    cpu->ime = true;
}

/* OPDA JP C a16 */
static void OP_DA(cpu_t* cpu, uint16_t addr ) {
    if(get_carry(cpu->reg)) {
        cpu->reg->pc = addr;
        // extra cycles
        cpu->clock_cycle+=4;
    }
}

/* OPDC CALL C a16 */
static void OP_DC(cpu_t* cpu, uint16_t addr) {
    if(get_carry(cpu->reg)) {
        stack_push(cpu, cpu->reg->pc);
        cpu->reg->pc = addr;
        // extra cycles
        cpu->clock_cycle+=12;
    }
}

/* OPDE SBC A d8 */
static void OP_DE(cpu_t* cpu, uint8_t val) {
    cpu->reg->a = alu_sbc8(cpu->reg, cpu->reg->a, val);
}

/* OPDF RST 18H */
static void OP_DF(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = 0x0018;
}

/* OPE0 LDH(a8) A */
static void OP_E0(cpu_t* cpu, uint8_t addr) {
    mmu_write_addr8(cpu->mmu, 0xFF00 + addr, cpu->reg->a);
}

/* OPE1 POP HL */
static void OP_E1(cpu_t* cpu) {
    cpu->reg->hl = stack_pop(cpu);
}

/* OPE2 LD (C), A */
static void OP_E2(cpu_t* cpu) {
    uint16_t addr = 0xFF00 + cpu->reg->c;
    mmu_write_addr8(cpu->mmu, addr, cpu->reg->a);
}

/* OPE5 PUSH HL */
static void OP_E5(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->hl);
}

/* OPE6 ADD AND d8*/
static void OP_E6(cpu_t* cpu, uint8_t val) {
    cpu->reg->a = cpu->reg->a & val;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    } else {
        reset_zero(cpu->reg);
    }

    set_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);

}

/* OPE7 RST 20H */
static void OP_E7(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = 0x20;
}

/* OPE8 ADD SP r8 */
static void OP_E8(cpu_t* cpu, char addr) {
    int signed_result = (int)cpu->reg->sp + addr;
    uint16_t result = (uint16_t)signed_result;
    if(signed_result > 0xFFFF) {
        set_carry(cpu->reg);
    } else {
        reset_carry(cpu->reg);
    }
    // Not too sure about this?
    if(should_add_halfcarry16(cpu->reg->sp, addr)) {
        set_halfcarry(cpu->reg);
    } else {
        reset_halfcarry(cpu->reg); 
    }

    cpu->reg->sp = result;
}

/* OPE9 JP (HL) */
static void OP_E9(cpu_t* cpu) {
    cpu->reg->pc = cpu->reg->hl;
}
/* OPEA LD (a16) A */
static void OP_EA(cpu_t* cpu, uint16_t addr) {
    mmu_write_addr8(cpu->mmu, addr, cpu->reg->a);
}
/* OPEE XOR d8 */
static void OP_EE(cpu_t* cpu, uint8_t val) {
    
    cpu->reg->a = cpu->reg->a ^ val;
    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}
/* OPEF RST 28H */
static void OP_EF(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = 0x28;
}
/* OPF0 LDH A (a8) */
static void OP_F0(cpu_t* cpu, uint8_t addr) {
    cpu->reg->a = mmu_read_addr8(cpu->mmu, 0xFF00 + addr);
}
/* OPF1 POP AF */
static void OP_F1(cpu_t* cpu) {
    // The lower nibble of flag reg is unused
    cpu->reg->af = stack_pop(cpu) & 0xFFF0;

}
/* OPF2 LD A (C) */
static void OP_F2(cpu_t* cpu) {
    uint16_t addr = cpu->reg->c + 0xFF00;
    cpu->reg->a = mmu_read_addr8(cpu->mmu, addr);
}
/* OPF3 DI */
static void OP_F3(cpu_t* cpu) {
    cpu->ime = false;
}
/* OPF5 PUSH AF*/
static void OP_F5(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->af);
}
/* OPF6 OR d8 */
static void OP_F6(cpu_t* cpu, uint8_t val) {
    cpu->reg->a = cpu->reg->a | val;

    if(cpu->reg->a == 0) {
        set_zero(cpu->reg);
    }

    reset_halfcarry(cpu->reg);
    reset_carry(cpu->reg);
    reset_subtract(cpu->reg);
}

/* OPF7 RST 30H */
static void OP_F7(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = 0x30;
}
/* OPF8 LD HL SP+r8 */
static void OP_F8(cpu_t* cpu, uint8_t addr) {
    // reset some flags
    reset_zero(cpu->reg);
    reset_subtract(cpu->reg);

    int val = cpu->reg->sp + (signed char)addr;
    if(val & 0xFFFF) {
        set_carry(cpu->reg);
    } else {
        reset_carry(cpu->reg);
    }

    if(((cpu->reg->sp & 0x0f) + (addr & 0x0f)) > 0x0f) {
        set_halfcarry(cpu->reg);
    } else {
        reset_halfcarry(cpu->reg);
    }
    cpu->reg->hl = val;
}

/* OPF9 LD SP HL */
static void OP_F9(cpu_t* cpu) {
    cpu->reg->sp = cpu->reg->hl;
}

/* OPFA LD A(a16) */
static void OP_FA(cpu_t* cpu, uint16_t addr) {
    cpu->reg->a = mmu_read_addr8(cpu->mmu, addr);
}

/* OPFB EI */
static void OP_FB(cpu_t* cpu) {
   cpu->ime = false;
}

/* OPFE CP d8 */
static void OP_FE(cpu_t* cpu, uint8_t val) {
    cp(cpu->reg, val);
}

/* OPFF RST 38H */
static void OP_FF(cpu_t* cpu) {
    stack_push(cpu, cpu->reg->pc);
    cpu->reg->pc = 0x38;
}

// IMPL stolen from: https://github.com/CTurt/Cinoop
const struct op_t_ ops[256] = {
    { "NOP", 0, OP_00, 4 },
    { "LD BC d16", 2, OP_01, 12 },
    { "LD (BC) A", 0, OP_02, 8 },
    { "INC BC", 0, OP_03, 8 },
    { "INC B", 0, OP_04, 4 },
    { "DEC B", 0, OP_05, 4 },
    { "LD B d8", 1, OP_06, 8 },
    { "RLCA", 0, OP_07, 4 },
    { "LD (a16) SP", 2, OP_08, 20 },
    { "ADD HL BC", 0, OP_09, 8 },
    { "LD A (BC)", 0, OP_0A, 8 },
    { "DEC BC", 0, OP_0B, 8 },
    { "INC C", 0, OP_0C, 4 },
    { "DEC C", 0, OP_0D, 4 },
    { "LD C d8", 1, OP_0E, 8 },
    { "RRCA", 0, OP_0F, 4 },
    { "STOP d8", 1, OP_10, 4 },
    { "LD DE d16", 2, OP_11, 12 },
    { "LD (DE) A ", 0, OP_12, 8 },
    { "INC DE", 0, OP_13, 8 },
    { "INC D", 0, OP_14, 4 },
    { "DEC D", 0, OP_15, 4 },
    { "LD D d8", 1, OP_16, 8  },
    { "RLA", 0, OP_17, 4 },
    { "JR r8", 1, OP_18, 12 },
    { "ADD HL DE", 0, OP_19, 8 },
    { "LD A (DE)", 0, OP_1A, 8 },
    { "DEC DE", 0, OP_1B, 8 },
    { "INC E", 0, OP_1C, 4 },
    { "DEC E", 0, OP_1D, 4 },
    { "LD E d8", 1, OP_1E, 8 },
    { "RRA", 0, OP_1F, 4 },
    { "JR NZ r8", 1, OP_20, 8 },
    { "LD HL 16", 2, OP_21, 12 },
    { "LD (HL+) A", 0, OP_22, 8 },
    { "INC HL", 0, OP_23, 8 },
    { "INC H", 0, OP_24, 4 },
    { "DEC H", 0, OP_25, 4 },
    { "LD H d8", 1, OP_26, 8 },
    { "DAA", 0, OP_27, 4 },
    { "JR Z r8", 1, OP_28, 8 },
    { "ADD HL, HL", 0, OP_29, 8 },
    { "LD A (HL+)", 0, OP_2A, 8 },
    { "DEC HL", 0, OP_2B, 8 },
    { "INC L", 0, OP_2C, 4 },
    { "DEC L", 0, OP_2D, 4 },
    { "LD L d8", 1, OP_2E, 8 },
    { "CPL", 0,  OP_2F, 4 },
    { "JR NC r8", 1, OP_30, 8 },
    { "LD SP d16", 2, OP_31, 12 },
    { "LD (HL-) A", 0, OP_32, 8 },
    { "INC SP", 0, OP_33, 8 }, 
    { "INC (HL)", 0, OP_34, 12 },
    { "DEC (HL)", 0, OP_35, 12 },
    { "LD (HL) d8", 1, OP_36, 12 }, 
    { "SCF", 0, OP_37, 4 },
    { "JR C r8", 1, OP_38, 8 }, 
    { "ADD HL SP", 0, OP_39, 8 }, 
    { "LD A (HL-)", 0, OP_3A, 8 },
    { "DEC SP", 0, OP_3B, 8 }, 
    { "INC A", 0, OP_3C, 4 },
    { "DEC A", 0, OP_3D, 4 },
    { "LD A d8", 1, OP_3E, 8 }, 
    { "CCF", 0, OP_3F, 4 },
    { "LD B B", 0, OP_40, 4 },
    { "LD B C", 0, OP_41, 4 },
    { "LD B D", 0, OP_42, 4 },
    { "LD B E", 0, OP_43, 4 },
    { "LD B H", 0, OP_44, 4 },
    { "LD B L", 0, OP_45, 4 },
    { "LD B (HL)", 0, OP_46, 8 },
    { "LD B A", 0, OP_47, 4 },
    { "LD C B", 0, OP_48, 4 },
    { "LD C C", 0, OP_49, 4 },
    { "LD C D", 0, OP_4A, 4 },
    { "LD C E", 0, OP_4B, 4 },
    { "LD C H", 0, OP_4C, 4 },
    { "LD C L", 0, OP_4D, 4 },
    { "LD C (HL)", 0, OP_4E, 8 },
    { "LD C A", 0, OP_4F, 4 },
    { "LD D B", 0, OP_50, 4 },
    { "LD D C", 0, OP_51, 4 },
    { "LD D D", 0, OP_52, 4 },
    { "LD D E", 0, OP_53, 4 },
    { "LD D H", 0, OP_54, 4 },
    { "LD D L", 0, OP_55, 4 },
    { "LD D (HL)", 0, OP_56, 8 },
    { "LD D A", 0, OP_57, 4 },
    { "LD E B", 0, OP_58, 4 },
    { "LD E C", 0, OP_59, 4 },
    { "LD E D", 0, OP_5A, 4 },
    { "LD E E", 0, OP_5B, 4 },
    { "LD E H", 0, OP_5C, 4 },
    { "LD E L", 0, OP_5D, 4 },
    { "LD E (HL)", 0, OP_5E, 8 },
    { "LD E A", 0, OP_5F, 4 },
    { "LD H B", 0, OP_60, 4 },
    { "LD H C", 0, OP_61, 4 }, 
    { "LD H D", 0, OP_62, 4 }, 
    { "LD H E", 0, OP_63, 4 }, 
    { "LD H H", 0, OP_64, 4 }, 
    { "LD H L", 0, OP_65, 4 }, 
    { "LD H (HL)", 0, OP_66, 8 }, 
    { "LD H A", 0, OP_67, 4 }, 
    { "LD L B", 0, OP_68, 4 }, 
    { "LD L C", 0, OP_69, 4 }, 
    { "LD L D", 0, OP_6A, 4 }, 
    { "LD L E", 0, OP_6B, 4 }, 
    { "LD L H", 0, OP_6C, 4 }, 
    { "LD L L", 0, OP_6D, 4 }, 
    { "LD L (HL)", 0, OP_6E, 8 }, 
    { "LD L, A", 0, OP_6F, 4 },
    { "LD (HL) B", 0, OP_70, 8 },
    { "LD (HL) C", 0, OP_71, 8 },
    { "LD (HL) D", 0, OP_72, 8 },
    { "LD (HL) E", 0, OP_73, 8 },
    { "LD (HL) H", 0, OP_74, 8 },
    { "LD (HL) L", 0, OP_75, 8 },
    { "HALT", 0, OP_76, 4 },
    { "LD (HL) A", 0, OP_77, 8 },
    { "LD A B", 0, OP_78, 4 },
    { "LD A C", 0, OP_79, 4 },
    { "LD A D", 0, OP_7A, 4 },
    { "LD L E", 0, OP_7B, 4 },
    { "LD L H", 0, OP_7C, 4 },
    { "LD L L", 0, OP_7D, 4 },
    { "LD A (HL)", 0, OP_7E, 8 },
    { "LD A A", 0, OP_7F, 4 },
    { "ADD A B", 0, OP_80, 4 },
    { "ADD A C", 0, OP_81, 4 },
    { "ADD A D", 0, OP_82, 4 },
    { "ADD A E", 0, OP_83, 4 },
    { "ADD A H", 0, OP_84, 4 },
    { "ADD A L", 0, OP_85, 4 },
    { "ADD A (HL)", 0, OP_86, 8 },
    { "ADD A A", 0, OP_87, 4 },
    { "ADC A B", 0, OP_88, 4 },
    { "ADC A C", 0, OP_89, 4 },
    { "ADC A D", 0, OP_8A, 4 },
    { "ADC A E", 0, OP_8B, 4 },
    { "ADC A H", 0, OP_8C, 4 },
    { "ADC A L", 0, OP_8D, 4 },
    { "ADC A (HL)", 0, OP_8E, 8 },
    { "ADC A A", 0, OP_8F, 4 },
    { "SUB B", 0, OP_90, 4 },
    { "SUB C", 0, OP_91, 4 },
    { "SUB D", 0, OP_92, 4 },
    { "SUB E", 0, OP_93, 4 },
    { "SUB H", 0, OP_94, 4 },
    { "SUB L", 0, OP_95, 4 },
    { "SUB (HL)", 0, OP_96, 8 },
    { "SUB A", 0, OP_97, 4 },
    { "SBC A B", 0, OP_98, 4 },
    { "SBC A C", 0, OP_99, 4 },
    { "SBC A D", 0, OP_9A, 4 },
    { "SBC A E", 0, OP_9B, 4 },
    { "SBC A H", 0, OP_9C, 4 },
    { "SBC A L", 0, OP_9D, 4 },
    { "SBC A (HL)", 0, OP_9E, 8 },
    { "SBC A A", 0, OP_9F, 4 },
    { "AND B", 0, OP_A0, 4 },
    { "AND C", 0, OP_A1, 4 },
    { "AND D", 0, OP_A2, 4 },
    { "AND E", 0, OP_A3, 4 },
    { "AND H", 0, OP_A4, 4 },
    { "AND L", 0, OP_A5, 4 },
    { "AND (HL)", 0, OP_A6, 8 },
    { "AND A", 0, OP_A7, 4 },
    { "XOR B", 0, OP_A8, 4 },
    { "XOR C", 0, OP_A9, 4 },
    { "XOR D", 0, OP_AA, 4 },
    { "XOR E", 0, OP_AB, 4 },
    { "XOR H", 0, OP_AC, 4 },
    { "XOR L", 0, OP_AD, 4 },
    { "XOR (HL)", 0, OP_AE, 8 },
    { "XOR A", 0, OP_AF, 4 },
    { "OR B", 0, OP_B0, 4 },
    { "OR C", 0, OP_B1, 4 },
    { "OR D", 0, OP_B2, 4 },
    { "OR E", 0, OP_B3, 4 },
    { "OR H", 0, OP_B4, 4 },
    { "OR L", 0, OP_B5, 4 },
    { "OR (HL)", 0, OP_B6, 8 },
    { "OR A", 0, OP_B7, 4 },
    { "CP B", 0, OP_B8, 4 },
    { "CP C", 0, OP_B9, 4 },
    { "CP D", 0, OP_BA, 4 },
    { "CP E", 0, OP_BB, 4 },
    { "CP H", 0, OP_BC, 4 },
    { "CP L", 0, OP_BD, 4 },
    { "CP (HL)", 0, OP_BE, 8 },
    { "CP A", 0, OP_BF, 4 },
    { "RET NZ", 0, OP_C0, 8 },
    { "POP BC", 0, OP_C1, 12 },
    { "JP NZ a16", 2, OP_C2, 12 },
    { "JP a16", 2, OP_C3, 16 },
    { "CALL NZ,a16", 2, OP_C4, 12 },
    { "PUSH BC", 0, OP_C5, 16 },
    { "ADD A d8", 1, OP_C6, 8 },
    { "RST 00H", 0, OP_C7, 16 },
    { "RETZ", 0, OP_C8, 8 },
    { "RET", 0, OP_C9, 16 },
    { "JP Z a16", 2, OP_CA, 12 },
    { "PREFIX CB", 1, OP_CB, 4 },
    { "CALL Z a16", 2, OP_CC, 12 },
    { "CALL a16", 2, OP_CD, 24 },
    { "ADC A d8", 1, OP_CE, 8 },
    { "RST 08H", 0, OP_CF, 16 },
    { "RET NC", 0, OP_D0, 8 },
    { "POP DE", 0, OP_D1, 12 },
    { "JP NC a16", 2, OP_D2, 12 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "CALL NC a16", 2, OP_D4, 12 },
    { "PUSH DE", 0, OP_D5, 16 },
    { "SUB d8", 0, OP_D6, 8 },
    { "RST 10H", 0, OP_D7, 16},
    { "RET C", 0, OP_D8, 8 },
    { "RETI", 0, OP_D9, 16 },
    { "JP C a16", 2, OP_DA, 12 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "CALL c a16", 2, OP_DC, 12 },
    { "UNDEFINED", 0, unknown_opcode, 0  },
    { "SBC A d8", 1, OP_DE, 8 },
    { "RST 18H", 0, OP_DF, 16 },
    { "LDH (a8) A", 1, OP_E0, 12 },
    { "POP HL", 0, OP_E1, 12 },
    { "LD (C) A", 0, OP_E2, 8 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "PUSH HL", 0, OP_E5, 16 },
    { "AND d8", 1, OP_E6, 8 },
    { "RST 20H", 0, OP_E7, 16 },
    { "ADD SP r8", 1, OP_E8, 16 },
    { "JP (HL)", 0, OP_E9, 4 },
    { "LD (a16) A", 2, OP_EA, 16 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "XOR d8", 1, OP_EE, 8 },
    { "RST 28H", 0, OP_EF, 16 },
    { "LDH A (a8)", 1, OP_F0, 12 },
    { "POP AF", 0, OP_F1, 12 },
    { "LD A (C)", 0, OP_F2, 8 },
    { "DI", 0, OP_F3, 4 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "PUSH AF", 0, OP_F5, 16 },
    { "OR d8", 1, OP_F6, 8 },
    { "RST 30H", 0, OP_F7, 16 },
    { "LD HL SP+r8", 1, OP_F8, 12 },
    { "LD SP HL", 0, OP_F9, 8 },
    { "LD A (a16)", 2, OP_FA, 16 },
    { "EI", 0, OP_FB, 4 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "UNDEFINED", 0, unknown_opcode, 0 },
    { "CP d8", 1, OP_FE, 8 },
    { "RST 38H", 0, OP_FF, 16 }
}; 
