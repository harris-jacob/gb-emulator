#include "cpu.h"
#include "register.h"
#include "mmu.h"


cpu_t* cpu_create() {
    reg_t* reg = reg_create();
    mmu_t* mmu = mmu_create();
    cpu_t cpu;

    cpu.mmu = mmu;
    cpu.reg = reg;
}

void cpu_destroy(cpu_t	**cpu) {
	free(*cpu);
	*cpu = NULL;
}

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
    // TODO
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

/* OP0F - LD, C, d8 */
static void OP_0F(cpu_t* cpu, uint8_t val) {
    // TODO
}

/* OP10 - LD, C, d8 */
static void OP_10(cpu_t* cpu, uint8_t val) {
    // TODO
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
    uint16_t val = alu_add16(cpu->reg, cpu->reg->de, 1);
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
    // TODO
}

/* OP18 - JR r8 */
static void OP_18(cpu_t* cpu) {
    // TODO
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
    // TODO
}

/* OP20 - JR NZ r8 */
static void OP_20(cpu_t* cpu) {
    // TODO
}

/* OP21 - LD HL d16 */
static void OP_21(cpu_t* cpu, uint16_t val) {
    cpu->reg->hl = val; 
}

/* OP22 - LD (DE) A */
static void OP_22(cpu_t* cpu) {
    mmu_write_addr8(cpu->mmu, cpu->reg->de, cpu->reg->a);
}

/* OP23 - INC DE */
static void OP_23(cpu_t* cpu) {
    cpu->reg->de++;
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
    //TODO
}

/* OP28 - JR Z, r8 */
static void OP_28(cpu_t* cpu) {
    // TODO
}

/* OP29 - ADD HL HL */
static void OP_29(cpu_t* cpu) {
    cpu->reg->hl = alu_add16(cpu->reg, cpu->reg->hl, cpu->reg->hl);
}

/* OP2A - LD A (HL+) */
static void OP_2A(cpu_t* cpu) {
    cpu->reg->a = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
    cpu->reg->hl++;
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

/* OP2F - LD L  */
static void OP_2F(cpu_t* cpu) {
    // CPL
}

/* OP30 - JR NC r8 */ 
static void OP_30(cpu_t* cpu) {
    //TODO
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
    uint8_t newVal = alu_de8(cpu->reg, val);
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, newVal);
}

/* OP36- LD (HL) d8 */
static void OP_36(cpu_t* cpu, uint8_t val) {
    mmu_write_addr8(cpu->mmu, cpu->reg->hl, val);
}

/* OP37 - SCF */
static void OP_37(cpu_t* cpu, uint8_t val) {
    // TODO
}

/* OP38 - JR C, r8 */
static void OP_38(cpu_t* cpu, uint8_t val) {
    // TODO
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

/* OP38 - JR C, r8 */
static void OP_3F(cpu_t* cpu, uint8_t val) {
    // TODO
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
static void OP_4D(cpu_t* cpu) {
    cpu->reg->c = mmu_read_addr8(cpu->mmu, cpu->reg->hl);
}

/* OP4F- LD C, A */
static void OP_4D(cpu_t* cpu) {
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
static void OP_6E(cpu_t* cpu) {
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
    // TODO
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