#include "cpu.h"


// RLC OPS
static void OP_00(cpu_t* cpu) { cpu->reg->b = rlc(cpu->reg, cpu->reg->b); };
static void OP_01(cpu_t* cpu) { cpu->reg->c = rlc(cpu->reg, cpu->reg->c); };
static void OP_02(cpu_t* cpu) { cpu->reg->d = rlc(cpu->reg, cpu->reg->d); };
static void OP_03(cpu_t* cpu) { cpu->reg->e = rlc(cpu->reg, cpu->reg->e); };
static void OP_04(cpu_t* cpu) { cpu->reg->h = rlc(cpu->reg, cpu->reg->h); };
static void OP_05(cpu_t* cpu) { cpu->reg->l = rlc(cpu->reg, cpu->reg->l); }; 
static void OP_06(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, rlc(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl)));};
static void OP_07(cpu_t* cpu) { cpu->reg->a = rlc(cpu->reg, cpu->reg->a); };

// RRC OPS
static void OP_08(cpu_t* cpu) { cpu->reg->b = rrc(cpu->reg, cpu->reg->b); };
static void OP_09(cpu_t* cpu) { cpu->reg->c = rrc(cpu->reg, cpu->reg->c); };
static void OP_0A(cpu_t* cpu) { cpu->reg->d = rrc(cpu->reg, cpu->reg->d); };
static void OP_0B(cpu_t* cpu) { cpu->reg->e = rrc(cpu->reg, cpu->reg->e); };
static void OP_0C(cpu_t* cpu) { cpu->reg->h = rrc(cpu->reg, cpu->reg->h); };
static void OP_0D(cpu_t* cpu) { cpu->reg->l = rrc(cpu->reg, cpu->reg->l); }; 
static void OP_0E(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, rrc(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl)));};
static void OP_0F(cpu_t* cpu) { cpu->reg->a = rrc(cpu->reg, cpu->reg->a); };

// RL OPS
static void OP_10(cpu_t* cpu) { cpu->reg->b = rl(cpu->reg, cpu->reg->b); };
static void OP_11(cpu_t* cpu) { cpu->reg->c = rl(cpu->reg, cpu->reg->c); };
static void OP_12(cpu_t* cpu) { cpu->reg->d = rl(cpu->reg, cpu->reg->d); };
static void OP_13(cpu_t* cpu) { cpu->reg->e = rl(cpu->reg, cpu->reg->e); };
static void OP_14(cpu_t* cpu) { cpu->reg->h = rl(cpu->reg, cpu->reg->h); };
static void OP_15(cpu_t* cpu) { cpu->reg->l = rl(cpu->reg, cpu->reg->l); }; 
static void OP_16(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, rl(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl)));};
static void OP_17(cpu_t* cpu) { cpu->reg->a = rl(cpu->reg, cpu->reg->a); };

// RR OPS
static void OP_18(cpu_t* cpu) { cpu->reg->b = rr(cpu->reg, cpu->reg->b); };
static void OP_19(cpu_t* cpu) { cpu->reg->c = rr(cpu->reg, cpu->reg->c); };
static void OP_1A(cpu_t* cpu) { cpu->reg->d = rr(cpu->reg, cpu->reg->d); };
static void OP_1B(cpu_t* cpu) { cpu->reg->e = rr(cpu->reg, cpu->reg->e); };
static void OP_1C(cpu_t* cpu) { cpu->reg->h = rr(cpu->reg, cpu->reg->h); };
static void OP_1D(cpu_t* cpu) { cpu->reg->l = rr(cpu->reg, cpu->reg->l); }; 
static void OP_1E(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, rr(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl)));};
static void OP_1F(cpu_t* cpu) { cpu->reg->a = rr(cpu->reg, cpu->reg->a); };

// SLA OPS
static void OP_20(cpu_t* cpu) { cpu->reg->b = sla(cpu->reg, cpu->reg->b); };
static void OP_21(cpu_t* cpu) { cpu->reg->c = sla(cpu->reg, cpu->reg->c); };
static void OP_22(cpu_t* cpu) { cpu->reg->d = sla(cpu->reg, cpu->reg->d); };
static void OP_23(cpu_t* cpu) { cpu->reg->e = sla(cpu->reg, cpu->reg->e); };
static void OP_24(cpu_t* cpu) { cpu->reg->h = sla(cpu->reg, cpu->reg->h); };
static void OP_25(cpu_t* cpu) { cpu->reg->l = sla(cpu->reg, cpu->reg->l); }; 
static void OP_26(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, sla(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl)));};
static void OP_27(cpu_t* cpu) { cpu->reg->a = sla(cpu->reg, cpu->reg->a); };

// SRA OPS
static void OP_28(cpu_t* cpu) { cpu->reg->b = sra(cpu->reg, cpu->reg->b); };
static void OP_29(cpu_t* cpu) { cpu->reg->c = sra(cpu->reg, cpu->reg->c); };
static void OP_2A(cpu_t* cpu) { cpu->reg->d = sra(cpu->reg, cpu->reg->d); };
static void OP_2B(cpu_t* cpu) { cpu->reg->e = sra(cpu->reg, cpu->reg->e); };
static void OP_2C(cpu_t* cpu) { cpu->reg->h = sra(cpu->reg, cpu->reg->h); };
static void OP_2D(cpu_t* cpu) { cpu->reg->l = sra(cpu->reg, cpu->reg->l); }; 
static void OP_2E(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, sra(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl)));};
static void OP_2F(cpu_t* cpu) { cpu->reg->a = sra(cpu->reg, cpu->reg->a); };

// SWAP OPS
static void OP_30(cpu_t* cpu) { cpu->reg->b = swap(cpu->reg, cpu->reg->b); };
static void OP_31(cpu_t* cpu) { cpu->reg->c = swap(cpu->reg, cpu->reg->c); };
static void OP_32(cpu_t* cpu) { cpu->reg->d = swap(cpu->reg, cpu->reg->d); };
static void OP_33(cpu_t* cpu) { cpu->reg->e = swap(cpu->reg, cpu->reg->e); };
static void OP_34(cpu_t* cpu) { cpu->reg->h = swap(cpu->reg, cpu->reg->h); };
static void OP_35(cpu_t* cpu) { cpu->reg->l = swap(cpu->reg, cpu->reg->l); }; 
static void OP_36(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, swap(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl)));};
static void OP_37(cpu_t* cpu) { cpu->reg->a = swap(cpu->reg, cpu->reg->a); };

// SRL OPS
static void OP_38(cpu_t* cpu) { cpu->reg->b = srl(cpu->reg, cpu->reg->b); };
static void OP_39(cpu_t* cpu) { cpu->reg->c = srl(cpu->reg, cpu->reg->c); };
static void OP_3A(cpu_t* cpu) { cpu->reg->d = srl(cpu->reg, cpu->reg->d); };
static void OP_3B(cpu_t* cpu) { cpu->reg->e = srl(cpu->reg, cpu->reg->e); };
static void OP_3C(cpu_t* cpu) { cpu->reg->h = srl(cpu->reg, cpu->reg->h); };
static void OP_3D(cpu_t* cpu) { cpu->reg->l = srl(cpu->reg, cpu->reg->l); }; 
static void OP_3E(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, srl(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl)));};
static void OP_3F(cpu_t* cpu) { cpu->reg->a = srl(cpu->reg, cpu->reg->a); };

// BIT 0 OPS
static void OP_40(cpu_t* cpu) { bit(cpu->reg, cpu->reg->b, 0); };
static void OP_41(cpu_t* cpu) { bit(cpu->reg, cpu->reg->c, 0); };
static void OP_42(cpu_t* cpu) { bit(cpu->reg, cpu->reg->d, 0); };
static void OP_43(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 0); };
static void OP_44(cpu_t* cpu) { bit(cpu->reg, cpu->reg->h, 0); };
static void OP_45(cpu_t* cpu) { bit(cpu->reg, cpu->reg->l, 0); }; 
static void OP_46(cpu_t* cpu) { bit(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl), 0);};
static void OP_47(cpu_t* cpu) { bit(cpu->reg, cpu->reg->a, 0); };

// BIT 1 OPS
static void OP_48(cpu_t* cpu) { bit(cpu->reg, cpu->reg->b, 1); };
static void OP_49(cpu_t* cpu) { bit(cpu->reg, cpu->reg->c, 1); };
static void OP_4A(cpu_t* cpu) { bit(cpu->reg, cpu->reg->d, 1); };
static void OP_4B(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 1); };
static void OP_4C(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 1); };
static void OP_4D(cpu_t* cpu) { bit(cpu->reg, cpu->reg->l, 1); }; 
static void OP_4E(cpu_t* cpu) { bit(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl), 1);};
static void OP_4F(cpu_t* cpu) { bit(cpu->reg, cpu->reg->a, 1); };

// BIT 2 OPS
static void OP_50(cpu_t* cpu) { bit(cpu->reg, cpu->reg->b, 2); };
static void OP_51(cpu_t* cpu) { bit(cpu->reg, cpu->reg->c, 2); };
static void OP_52(cpu_t* cpu) { bit(cpu->reg, cpu->reg->d, 2); };
static void OP_53(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 2); };
static void OP_54(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 2); };
static void OP_55(cpu_t* cpu) { bit(cpu->reg, cpu->reg->l, 2); }; 
static void OP_56(cpu_t* cpu) { bit(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl), 2);};
static void OP_57(cpu_t* cpu) { bit(cpu->reg, cpu->reg->a, 2); };

// BIT 3 OPS
static void OP_58(cpu_t* cpu) { bit(cpu->reg, cpu->reg->b, 3); };
static void OP_59(cpu_t* cpu) { bit(cpu->reg, cpu->reg->c, 3); };
static void OP_5A(cpu_t* cpu) { bit(cpu->reg, cpu->reg->d, 3); };
static void OP_5B(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 3); };
static void OP_5C(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 3); };
static void OP_5D(cpu_t* cpu) { bit(cpu->reg, cpu->reg->l, 3); }; 
static void OP_5E(cpu_t* cpu) { bit(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl), 3);};
static void OP_5F(cpu_t* cpu) { bit(cpu->reg, cpu->reg->a, 3); };

// BIT 4 OPS
static void OP_60(cpu_t* cpu) { bit(cpu->reg, cpu->reg->b, 4); };
static void OP_61(cpu_t* cpu) { bit(cpu->reg, cpu->reg->c, 4); };
static void OP_62(cpu_t* cpu) { bit(cpu->reg, cpu->reg->d, 4); };
static void OP_63(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 4); };
static void OP_64(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 4); };
static void OP_65(cpu_t* cpu) { bit(cpu->reg, cpu->reg->l, 4); }; 
static void OP_66(cpu_t* cpu) { bit(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl), 4);};
static void OP_67(cpu_t* cpu) { bit(cpu->reg, cpu->reg->a, 4); };

// BIT 5 OPS
static void OP_68(cpu_t* cpu) { bit(cpu->reg, cpu->reg->b, 5); };
static void OP_69(cpu_t* cpu) { bit(cpu->reg, cpu->reg->c, 5); };
static void OP_6A(cpu_t* cpu) { bit(cpu->reg, cpu->reg->d, 5); };
static void OP_6B(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 5); };
static void OP_6C(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 5); };
static void OP_6D(cpu_t* cpu) { bit(cpu->reg, cpu->reg->l, 5); }; 
static void OP_6E(cpu_t* cpu) { bit(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl), 5);};
static void OP_6F(cpu_t* cpu) { bit(cpu->reg, cpu->reg->a, 5); };

// BIT 6 OPS
static void OP_70(cpu_t* cpu) { bit(cpu->reg, cpu->reg->b, 6); };
static void OP_71(cpu_t* cpu) { bit(cpu->reg, cpu->reg->c, 6); };
static void OP_72(cpu_t* cpu) { bit(cpu->reg, cpu->reg->d, 6); };
static void OP_73(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 6); };
static void OP_74(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 6); };
static void OP_75(cpu_t* cpu) { bit(cpu->reg, cpu->reg->l, 6); }; 
static void OP_76(cpu_t* cpu) { bit(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl), 6);};
static void OP_77(cpu_t* cpu) { bit(cpu->reg, cpu->reg->a, 6); };

// BIT 7 OPS
static void OP_78(cpu_t* cpu) { bit(cpu->reg, cpu->reg->b, 7); };
static void OP_79(cpu_t* cpu) { bit(cpu->reg, cpu->reg->c, 7); };
static void OP_7A(cpu_t* cpu) { bit(cpu->reg, cpu->reg->d, 7); };
static void OP_7B(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 7); };
static void OP_7C(cpu_t* cpu) { bit(cpu->reg, cpu->reg->e, 7); };
static void OP_7D(cpu_t* cpu) { bit(cpu->reg, cpu->reg->l, 7); }; 
static void OP_7E(cpu_t* cpu) { bit(cpu->reg, mmu_read_addr8(cpu->mmu, cpu->reg->hl), 7);};
static void OP_7F(cpu_t* cpu) { bit(cpu->reg, cpu->reg->a, 7); };

// RES 0 OPS
static void OP_80(cpu_t* cpu) { cpu->reg->b = reset(cpu->reg->b, 0); };
static void OP_81(cpu_t* cpu) { cpu->reg->c = reset(cpu->reg->c, 0); };
static void OP_82(cpu_t* cpu) { cpu->reg->d = reset(cpu->reg->d, 0); };
static void OP_83(cpu_t* cpu) { cpu->reg->e = reset(cpu->reg->e, 0); };
static void OP_84(cpu_t* cpu) { cpu->reg->h = reset(cpu->reg->h, 0); };
static void OP_85(cpu_t* cpu) { cpu->reg->l = reset(cpu->reg->l, 0); }; 
static void OP_86(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, reset(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 0));};
static void OP_87(cpu_t* cpu) { cpu->reg->a = reset(cpu->reg->a, 0); };

// RES 1 OPS
static void OP_88(cpu_t* cpu) { cpu->reg->b = reset(cpu->reg->b, 1); };
static void OP_89(cpu_t* cpu) { cpu->reg->c = reset(cpu->reg->c, 1); };
static void OP_8A(cpu_t* cpu) { cpu->reg->d = reset(cpu->reg->d, 1); };
static void OP_8B(cpu_t* cpu) { cpu->reg->e = reset(cpu->reg->e, 1); };
static void OP_8C(cpu_t* cpu) { cpu->reg->h = reset(cpu->reg->e, 1); };
static void OP_8D(cpu_t* cpu) { cpu->reg->l = reset(cpu->reg->l, 1); }; 
static void OP_8E(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, reset(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 1));};
static void OP_8F(cpu_t* cpu) { cpu->reg->a = reset(cpu->reg->a, 1); };

// RES 2 OPS
static void OP_90(cpu_t* cpu) { cpu->reg->b = reset(cpu->reg->b, 2); };
static void OP_91(cpu_t* cpu) { cpu->reg->c = reset(cpu->reg->c, 2); };
static void OP_92(cpu_t* cpu) { cpu->reg->d = reset(cpu->reg->d, 2); };
static void OP_93(cpu_t* cpu) { cpu->reg->e = reset(cpu->reg->e, 2); };
static void OP_94(cpu_t* cpu) { cpu->reg->h = reset(cpu->reg->e, 2); };
static void OP_95(cpu_t* cpu) { cpu->reg->l = reset(cpu->reg->l, 2); }; 
static void OP_96(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, reset(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 2));};
static void OP_97(cpu_t* cpu) { cpu->reg->a = reset(cpu->reg->a, 2); };

// RES 3 OPS
static void OP_98(cpu_t* cpu) { cpu->reg->b = reset(cpu->reg->b, 3); };
static void OP_99(cpu_t* cpu) { cpu->reg->c = reset(cpu->reg->c, 3); };
static void OP_9A(cpu_t* cpu) { cpu->reg->d = reset(cpu->reg->d, 3); };
static void OP_9B(cpu_t* cpu) { cpu->reg->e = reset(cpu->reg->e, 3); };
static void OP_9C(cpu_t* cpu) { cpu->reg->h = reset(cpu->reg->e, 3); };
static void OP_9D(cpu_t* cpu) { cpu->reg->l = reset(cpu->reg->l, 3); }; 
static void OP_9E(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, reset(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 3));};
static void OP_9F(cpu_t* cpu) { cpu->reg->a = reset(cpu->reg->a, 3); };

// RES 4 OPS
static void OP_A0(cpu_t* cpu) { cpu->reg->b = reset(cpu->reg->b, 4); };
static void OP_A1(cpu_t* cpu) { cpu->reg->c = reset(cpu->reg->c, 4); };
static void OP_A2(cpu_t* cpu) { cpu->reg->d = reset(cpu->reg->d, 4); };
static void OP_A3(cpu_t* cpu) { cpu->reg->e = reset(cpu->reg->e, 4); };
static void OP_A4(cpu_t* cpu) { cpu->reg->h = reset(cpu->reg->e, 4); };
static void OP_A5(cpu_t* cpu) { cpu->reg->l = reset(cpu->reg->l, 4); }; 
static void OP_A6(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, reset(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 4));};
static void OP_A7(cpu_t* cpu) { cpu->reg->a = reset(cpu->reg->a, 4); };

// RES 5 OPS
static void OP_A8(cpu_t* cpu) { cpu->reg->b = reset(cpu->reg->b, 5); };
static void OP_A9(cpu_t* cpu) { cpu->reg->c = reset(cpu->reg->c, 5); };
static void OP_AA(cpu_t* cpu) { cpu->reg->d = reset(cpu->reg->d, 5); };
static void OP_AB(cpu_t* cpu) { cpu->reg->e = reset(cpu->reg->e, 5); };
static void OP_AC(cpu_t* cpu) { cpu->reg->h = reset(cpu->reg->e, 5); };
static void OP_AD(cpu_t* cpu) { cpu->reg->l = reset(cpu->reg->l, 5); }; 
static void OP_AE(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, reset(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 5));};
static void OP_AF(cpu_t* cpu) { cpu->reg->a = reset(cpu->reg->a, 5); };

// RES 6 OPS
static void OP_B0(cpu_t* cpu) { cpu->reg->b = reset(cpu->reg->b, 6); };
static void OP_B1(cpu_t* cpu) { cpu->reg->c = reset(cpu->reg->c, 6); };
static void OP_B2(cpu_t* cpu) { cpu->reg->d = reset(cpu->reg->d, 6); };
static void OP_B3(cpu_t* cpu) { cpu->reg->e = reset(cpu->reg->e, 6); };
static void OP_B4(cpu_t* cpu) { cpu->reg->h = reset(cpu->reg->e, 6); };
static void OP_B5(cpu_t* cpu) { cpu->reg->l = reset(cpu->reg->l, 6); }; 
static void OP_B6(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, reset(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 6));};
static void OP_B7(cpu_t* cpu) { cpu->reg->a = reset(cpu->reg->a, 6); };

// RES 7 OPS
static void OP_B8(cpu_t* cpu) { cpu->reg->b = reset(cpu->reg->b, 7); };
static void OP_B9(cpu_t* cpu) { cpu->reg->c = reset(cpu->reg->c, 7); };
static void OP_BA(cpu_t* cpu) { cpu->reg->d = reset(cpu->reg->d, 7); };
static void OP_BB(cpu_t* cpu) { cpu->reg->e = reset(cpu->reg->e, 7); };
static void OP_BC(cpu_t* cpu) { cpu->reg->h = reset(cpu->reg->e, 7); };
static void OP_BD(cpu_t* cpu) { cpu->reg->l = reset(cpu->reg->l, 7); }; 
static void OP_BE(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, reset(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 7));};
static void OP_BF(cpu_t* cpu) { cpu->reg->a = reset(cpu->reg->a, 7); };


// SET 0 OPS
static void OP_C0(cpu_t* cpu) { cpu->reg->b = set(cpu->reg->b, 0); };
static void OP_C1(cpu_t* cpu) { cpu->reg->c = set(cpu->reg->c, 0); };
static void OP_C2(cpu_t* cpu) { cpu->reg->d = set(cpu->reg->d, 0); };
static void OP_C3(cpu_t* cpu) { cpu->reg->e = set(cpu->reg->e, 0); };
static void OP_C4(cpu_t* cpu) { cpu->reg->h = set(cpu->reg->h, 0); };
static void OP_C5(cpu_t* cpu) { cpu->reg->l = set(cpu->reg->l, 0); }; 
static void OP_C6(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, set(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 0));};
static void OP_C7(cpu_t* cpu) { cpu->reg->a = set(cpu->reg->a, 0); };

// SET 1 OPS
static void OP_C8(cpu_t* cpu) { cpu->reg->b = set(cpu->reg->b, 1); };
static void OP_C9(cpu_t* cpu) { cpu->reg->c = set(cpu->reg->c, 1); };
static void OP_CA(cpu_t* cpu) { cpu->reg->d = set(cpu->reg->d, 1); };
static void OP_CB(cpu_t* cpu) { cpu->reg->e = set(cpu->reg->e, 1); };
static void OP_CC(cpu_t* cpu) { cpu->reg->h = set(cpu->reg->e, 1); };
static void OP_CD(cpu_t* cpu) { cpu->reg->l = set(cpu->reg->l, 1); }; 
static void OP_CE(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, set(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 1));};
static void OP_CF(cpu_t* cpu) { cpu->reg->a = set(cpu->reg->a, 1); };

// SET 2 OPS
static void OP_D0(cpu_t* cpu) { cpu->reg->b = set(cpu->reg->b, 2); };
static void OP_D1(cpu_t* cpu) { cpu->reg->c = set(cpu->reg->c, 2); };
static void OP_D2(cpu_t* cpu) { cpu->reg->d = set(cpu->reg->d, 2); };
static void OP_D3(cpu_t* cpu) { cpu->reg->e = set(cpu->reg->e, 2); };
static void OP_D4(cpu_t* cpu) { cpu->reg->h = set(cpu->reg->e, 2); };
static void OP_D5(cpu_t* cpu) { cpu->reg->l = set(cpu->reg->l, 2); }; 
static void OP_D6(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, set(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 2));};
static void OP_D7(cpu_t* cpu) { cpu->reg->a = set(cpu->reg->a, 2); };

// SET 3 OPS
static void OP_D8(cpu_t* cpu) { cpu->reg->b = set(cpu->reg->b, 3); };
static void OP_D9(cpu_t* cpu) { cpu->reg->c = set(cpu->reg->c, 3); };
static void OP_DA(cpu_t* cpu) { cpu->reg->d = set(cpu->reg->d, 3); };
static void OP_DB(cpu_t* cpu) { cpu->reg->e = set(cpu->reg->e, 3); };
static void OP_DC(cpu_t* cpu) { cpu->reg->h = set(cpu->reg->e, 3); };
static void OP_DD(cpu_t* cpu) { cpu->reg->l = set(cpu->reg->l, 3); }; 
static void OP_DE(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, set(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 3));};
static void OP_DF(cpu_t* cpu) { cpu->reg->a = set(cpu->reg->a, 3); };

// SET 4 OPS
static void OP_E0(cpu_t* cpu) { cpu->reg->b = set(cpu->reg->b, 4); };
static void OP_E1(cpu_t* cpu) { cpu->reg->c = set(cpu->reg->c, 4); };
static void OP_E2(cpu_t* cpu) { cpu->reg->d = set(cpu->reg->d, 4); };
static void OP_E3(cpu_t* cpu) { cpu->reg->e = set(cpu->reg->e, 4); };
static void OP_E4(cpu_t* cpu) { cpu->reg->h = set(cpu->reg->e, 4); };
static void OP_E5(cpu_t* cpu) { cpu->reg->l = set(cpu->reg->l, 4); }; 
static void OP_E6(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, set(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 4));};
static void OP_E7(cpu_t* cpu) { cpu->reg->a = set(cpu->reg->a, 4); };

// SET 5 OPS
static void OP_E8(cpu_t* cpu) { cpu->reg->b = set(cpu->reg->b, 5); };
static void OP_E9(cpu_t* cpu) { cpu->reg->c = set(cpu->reg->c, 5); };
static void OP_EA(cpu_t* cpu) { cpu->reg->d = set(cpu->reg->d, 5); };
static void OP_EB(cpu_t* cpu) { cpu->reg->e = set(cpu->reg->e, 5); };
static void OP_EC(cpu_t* cpu) { cpu->reg->h = set(cpu->reg->e, 5); };
static void OP_ED(cpu_t* cpu) { cpu->reg->l = set(cpu->reg->l, 5); }; 
static void OP_EE(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, set(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 5));};
static void OP_EF(cpu_t* cpu) { cpu->reg->a = set(cpu->reg->a, 5); };

// SET 6 OPS
static void OP_F0(cpu_t* cpu) { cpu->reg->b = set(cpu->reg->b, 6); };
static void OP_F1(cpu_t* cpu) { cpu->reg->c = set(cpu->reg->c, 6); };
static void OP_F2(cpu_t* cpu) { cpu->reg->d = set(cpu->reg->d, 6); };
static void OP_F3(cpu_t* cpu) { cpu->reg->e = set(cpu->reg->e, 6); };
static void OP_F4(cpu_t* cpu) { cpu->reg->h = set(cpu->reg->e, 6); };
static void OP_F5(cpu_t* cpu) { cpu->reg->l = set(cpu->reg->l, 6); }; 
static void OP_F6(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, set(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 6));};
static void OP_F7(cpu_t* cpu) { cpu->reg->a = set(cpu->reg->a, 6); };

// SET 7 OPS
static void OP_F8(cpu_t* cpu) { cpu->reg->b = set(cpu->reg->b, 7); };
static void OP_F9(cpu_t* cpu) { cpu->reg->c = set(cpu->reg->c, 7); };
static void OP_FA(cpu_t* cpu) { cpu->reg->d = set(cpu->reg->d, 7); };
static void OP_FB(cpu_t* cpu) { cpu->reg->e = set(cpu->reg->e, 7); };
static void OP_FC(cpu_t* cpu) { cpu->reg->h = set(cpu->reg->e, 7); };
static void OP_FD(cpu_t* cpu) { cpu->reg->l = set(cpu->reg->l, 7); }; 
static void OP_FE(cpu_t* cpu) { mmu_write_addr8(cpu->mmu, cpu->reg->hl, set(mmu_read_addr8(cpu->mmu, cpu->reg->hl), 7));};
static void OP_FF(cpu_t* cpu) { cpu->reg->a = set(cpu->reg->a, 7); };

/* map of extended CB operations */
const struct extended_op_t_ extended_ops[256] = {
    { "RLC B", OP_00, 8 },
    { "RLC C", OP_01, 8},
    { "RLC D", OP_02, 8 },
    { "RLC E", OP_03, 8 },
    { "RLC H", OP_04, 8 },
    { "RLC F", OP_05, 8 },
    { "RLC (HL)", OP_06, 16 },
    { "RLC A", OP_07, 8 },
    { "RRC B", OP_08, 8 },
    { "RRC C", OP_09, 8 },
    { "RRC D", OP_0A, 8 },
    { "RRC E", OP_0B, 8 },
    { "RRC H", OP_0C, 8 },
    { "RRC F", OP_0D, 8 },
    { "RRC (HL)", OP_0E, 16 },
    { "RRC A", OP_0F, 8 },
    { "RL B", OP_10, 8 },
    { "RL C", OP_11, 8 },
    { "RL D", OP_12, 8 },
    { "RL E", OP_13, 8 },
    { "RL H", OP_14, 8 },
    { "RL F", OP_15, 8 },
    { "RL (HL)", OP_16, 16  },
    { "RL A", OP_17, 8 },
    { "RR B", OP_18, 8 },
    { "RR C", OP_19, 8 },
    { "RR D", OP_1A, 8 },
    { "RR E", OP_1B, 8 },
    { "RR H", OP_1C, 8 },
    { "RR L", OP_1D, 8 },
    { "RR (HL)", OP_1E, 16 },
    { "RR A", OP_1F, 8 },
    { "SLA B", OP_20, 8 },
    { "SLA C", OP_21, 8 },
    { "SLA D", OP_22, 8 },
    { "SLA E", OP_23, 8 },
    { "SLA H", OP_24, 8 },
    { "SLA L", OP_25, 8 },
    { "SLA (HL)", OP_26, 16 },
    { "SLA A",OP_27, 8 },
    { "SRA B", OP_28, 8 },
    { "SRA C", OP_29, 8 },
    { "SRA D", OP_2A, 8 },
    { "SRA E", OP_2B, 8 },
    { "SRA H", OP_2C, 8 },
    { "SRA L", OP_2D, 8 },
    { "SRA (HL)", OP_2E, 16 },
    { "SRA A", OP_2F, 8 },
    { "SWAP B", OP_30, 8 },
    { "SWAP C", OP_31, 8 },
    { "SWAP D", OP_32, 8 },
    { "SWAP E", OP_33, 8 }, 
    { "SWAP H", OP_34, 8 },
    { "SWAP L", OP_35, 8 },
    { "SWAP (HL)", OP_36, 16 }, 
    { "SWAP A", OP_37, 8 },
    { "SRL B", OP_38, 8 }, 
    { "SRL C", OP_39, 8 }, 
    { "SRL D", OP_3A, 8 },
    { "SRL E", OP_3B, 8 }, 
    { "SRL H", OP_3C, 8 },
    { "SRL L", OP_3D, 8 },
    { "SRL (HL)", OP_3E, 16 }, 
    { "SRL A", OP_3F, 8 },
    { "BIT 0 B", OP_40, 8 },
    { "BIT 0 C", OP_41, 8 },
    { "BIT 0 D", OP_42, 8 },
    { "BIT 0 E", OP_43, 8 },
    { "BIT 0 H", OP_44, 8 },
    { "BIT 0 L", OP_45, 8 },
    { "BIT 0 (HL)", OP_46, 16 },
    { "BIT 0 A", OP_47, 8 },
    { "BIT 1 B", OP_48, 8 },
    { "BIT 1 C", OP_49, 8 },
    { "BIT 1 D", OP_4A, 8 },
    { "BIT 1 E", OP_4B, 8 },
    { "BIT 1 H", OP_4C, 8 },
    { "BIT 1 L", OP_4D, 8 },
    { "BIT 1 (HL)", OP_4E, 16 },
    { "BIT 1 A", OP_4F, 8 },
    { "BIT 2 B", OP_50, 8 },
    { "BIT 2 C", OP_51, 8 },
    { "BIT 2 D", OP_52, 8 },
    { "BIT 2 E", OP_53, 8 },
    { "BIT 2 H", OP_54, 8 },
    { "BIT 2 L", OP_55, 8 },
    { "BIT 2 (HL)", OP_56, 16 },
    { "BIT 2 A", OP_57, 8 },
    { "BIT 3 B", OP_58, 8 },
    { "BIT 3 C", OP_59, 8 },
    { "BIT 3 D", OP_5A, 8 },
    { "BIT 3 E", OP_5B, 8 },
    { "BIT 3 H", OP_5C, 8 },
    { "BIT 3 L", OP_5D, 8 },
    { "BIT 3 (HL)", OP_5E, 16 },
    { "BIT 3 A", OP_5F, 8 },
    { "BIT 4 B", OP_60, 8 },
    { "BIT 4 C", OP_61, 8 }, 
    { "BIT 4 D", OP_62, 8 }, 
    { "BIT 4 E", OP_63, 8 }, 
    { "BIT 4 H", OP_64, 8 }, 
    { "BIT 4 L", OP_65, 8 }, 
    { "BIT 4 (HL)", OP_66, 16 }, 
    { "BIT 4 A", OP_67, 8 }, 
    { "BIT 5 B", OP_68, 8 }, 
    { "BIT 5 C", OP_69, 8 }, 
    { "BIT 5 D", OP_6A, 8 }, 
    { "BIT 5 E", OP_6B, 8 }, 
    { "BIT 5 H", OP_6C, 8 }, 
    { "BIT 5 L", OP_6D, 8 }, 
    { "BIT 5 (HL)",OP_6E, 16 }, 
    { "BIT 5 A", OP_6F, 8 },
    { "BIT 6 B", OP_70, 8 },
    { "BIT 6 C", OP_71, 8 },
    { "BIT 6 D", OP_72, 8 },
    { "BIT 6 E", OP_73, 8 },
    { "BIT 6 H", OP_74, 8 },
    { "BIT 6 L", OP_75, 8 },
    { "BIT 6 (HL)", OP_76, 16 },
    { "BIT 6 A", OP_77, 8 },
    { "BIT 7 B", OP_78, 8 },
    { "BIT 7 C", OP_79, 8 },
    { "BIT 7 D", OP_7A, 8 },
    { "BIT 7 E", OP_7B, 8 },
    { "BIT 7 H", OP_7C, 8 },
    { "BIT 7 L", OP_7D, 8 },
    { "BIT 7 (HL)", OP_7E, 16 },
    { "BIT 7 A", OP_7F, 8 },
    { "RES 0 B", OP_80, 8 },
    { "RES 0 C", OP_81, 8 },
    { "RES 0 D", OP_82, 8 },
    { "RES 0 E", OP_83, 8 },
    { "RES 0 H", OP_84, 8 },
    { "RES 0 L", OP_85, 8 },
    { "RES 0 (HL)", OP_86, 16 },
    { "RES 0 A", OP_87, 8 },
    { "RES 1 B", OP_88, 8 },
    { "RES 1 C", OP_89, 8 },
    { "RES 1 D", OP_8A, 8 },
    { "RES 1 E", OP_8B, 8 },
    { "RES 1 H", OP_8C, 8 },
    { "RES 1 L", OP_8D, 8 },
    { "RES 1 (HL)", OP_8E, 16 },
    { "RES 1 A", OP_8F, 8 },
    { "RES 2 B", OP_90, 8 },
    { "RES 2 C", OP_91, 8 },
    { "RES 2 D", OP_92, 8 },
    { "RES 2 E", OP_93, 8 },
    { "RES 2 H", OP_94, 8 },
    { "RES 2 L", OP_95, 8 },
    { "RES 2 (HL)", OP_96, 16 },
    { "RES 2 A", OP_97, 8 },
    { "RES 3 B", OP_98, 8 },
    { "RES 3 C", OP_99, 8 },
    { "RES 3 D", OP_9A, 8 },
    { "RES 3 E", OP_9B, 8 },
    { "RES 3 H", OP_9C, 8 },
    { "RES 3 L", OP_9D, 8 },
    { "RES 3 (HL)", OP_9E, 16 },
    { "RES 3 A", OP_9F, 8 },
    { "RES 4 B", OP_A0, 8 },
    { "RES 4 C", OP_A1, 8 },
    { "RES 4 D", OP_A2, 8 },
    { "RES 4 E", OP_A3, 8 },
    { "RES 4 H", OP_A4, 8 },
    { "RES 4 L", OP_A5, 8 },
    { "RES 4 (HL)", OP_A6, 16 },
    { "RES 4 A", OP_A7, 8 },
    { "RES 5 B", OP_A8, 8 },
    { "RES 5 C", OP_A9, 8 },
    { "RES 5 D", OP_AA, 8 },
    { "RES 5 E", OP_AB, 8 },
    { "RES 5 H", OP_AC, 8 },
    { "RES 5 L", OP_AD, 8 },
    { "RES 5 (HL)", OP_AE, 16 },
    { "RES 5 A", OP_AF, 8 },
    { "RES 6 B", OP_B0, 8 },
    { "RES 6 C", OP_B1, 8 },
    { "RES 6 D", OP_B2, 8 },
    { "RES 6 E", OP_B3, 8 },
    { "RES 6 H", OP_B4, 8 },
    { "RES 6 L", OP_B5, 8 },
    { "RES 6 (HL)", OP_B6, 16 },
    { "RES 6 A", OP_B7, 8 },
    { "RES 7 B", OP_B8, 8 },
    { "RES 7 C", OP_B9, 8 },
    { "RES 7 D", OP_BA, 8 },
    { "RES 7 E", OP_BB, 8 },
    { "RES 7 H", OP_BC, 8 },
    { "RES 7 L", OP_BD, 8 },
    { "RES 7 (HL)", OP_BE, 16 },
    { "RES 7 A", OP_BF, 8 },
    { "SET 0 B", OP_C0, 8 },
    { "SET 0 C", OP_C1, 8 },
    { "SET 0 D", OP_C2, 8 },
    { "SET 0 E", OP_C3, 8 },
    { "SET 0 H", OP_C4, 8 },
    { "SET 0 L", OP_C5, 8 },
    { "SET 0 (HL)", OP_C6, 16},
    { "SET 0 A", OP_C7, 8 },
    { "SET 1 B", OP_C8, 8 },
    { "SET 1 C", OP_C9, 8 },
    { "SET 1 D", OP_CA, 8 },
    { "SET 1 E", OP_CB, 8 },
    { "SET 1 H", OP_CC, 8 },
    { "SET 1 L", OP_CD, 8 },
    { "SET 1 (HL)", OP_CE, 16 },
    { "SET 1 A", OP_CF, 8 },
    { "SET 2 B", OP_D0, 8 },
    { "SET 2 C", OP_D1, 8 },
    { "SET 2 D", OP_D2, 8 },
    { "SET 2 E", OP_D3, 8 },
    { "SET 2 H", OP_D4, 8 },
    { "SET 2 L", OP_D5, 8 },
    { "SET 2 (HL)", OP_D6, 16 },
    { "SET 2 A", OP_D7, 8 },
    { "SET 3 B", OP_D8, 8 },
    { "SET 3 C", OP_D9, 8 },
    { "SET 3 D", OP_DA, 8 },
    { "SET 3 E", OP_DB, 8 },
    { "SET 3 H", OP_DC, 8 },
    { "SET 3 L", OP_DD, 8  },
    { "SET 3 (HL)", OP_DE, 16},
    { "SET 3 A", OP_DF, 8 },
    { "SET 4 B", OP_E0, 8 },
    { "SET 4 C", OP_E1, 8 },
    { "SET 4 D", OP_E2, 8 },
    { "SET 4 E", OP_E3, 8 },
    { "SET 4 H", OP_E4, 8 },
    { "SET 4 L", OP_E5, 8 },
    { "SET 4 (HL)", OP_E6, 16 },
    { "SET 4 A", OP_E7, 8 },
    { "SET 5 B", OP_E8, 8 },
    { "SET 5 C", OP_E9, 8 },
    { "SET 5 D", OP_EA, 8 },
    { "SET 5 E", OP_EB, 8 },
    { "SET 5 H", OP_EC, 8 },
    { "SET 5 L", OP_ED, 8 },
    { "SET 5 (HL)", OP_EE, 16 },
    { "SET 5 A", OP_EF, 8 },
    { "SET 6 B", OP_F0, 8 },
    { "SET 6 C", OP_F1, 8 },
    { "SET 6 D", OP_F2, 8 },
    { "SET 6 E", OP_F3, 8 },
    { "SET 6 H", OP_F4, 8 },
    { "SET 6 L", OP_F5, 8 },
    { "SET 6 (HL)", OP_F6, 16 },
    { "SET 6 A", OP_F7, 8 },
    { "SET 7 B", OP_F8, 8 },
    { "SET 7 C", OP_F9, 8 },
    { "SET 7 D", OP_FA, 8 },
    { "SET 7 E", OP_FB, 8 },
    { "SET 7 H", OP_FC, 8 },
    { "SET 7 L", OP_FD, 8 },
    { "SET 7 (HL)", OP_FE, 16 },
    { "SET 7 A", OP_FF, 8 }
}; 
