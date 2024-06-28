///Possible register pairs
#[derive(Copy, Clone)]
pub enum RegisterPair {
    BC, DE, HL,
}

///Possible address modes
#[derive(Copy, Clone)]
pub enum AddressMode {
    None,
    Register,
    Extended,
    Immediate,
    ImmediateExtended,
}

///Main instruction set
#[derive(Copy, Clone)]
pub enum Instruction {
    NOP, LDBCNN, _LDMBCA, _INCBC, INCB, DECB, LDBN, _RLCA, _EXAFAF, _ADDHLBC, _LDABCM, _DECBC, _INCC, _DECC, _LDCN, _RRCA,
    _DJNZD, LDDENN, _LDMDEA, _INCDE, _INCD, _DECD, _LDDN, _RLA, _JRD, _ADDHLDE, _LDADEM, _DECDE, _INCE, _DECE, _LDEN, _RRA,
    _JRNZD, LDHLNNM, LDMNNHL, INCHL, _INCH, _DECH, _LDHN, _DAA, JRZD, _ADDHLHL, _LDHLNNM, _DECHL, _INCL, _DECL, _LDLN, _CPL,
    _JRNCD, LDSPNN, _LDMNNA, _INCSP, _INCMHL, _DECMHL, LDMHLN, _SCF, _JRCD, _ADDHLSP, _LDANNM, _DECSP, _INCA, _DECA, LDAN, _CCF,
    _LDBB, _LDBC, _LDBD, _LDBE, _LDBH, _LDBL, _LDBHLM, LDBA, _LDCB, _LDCC, _LDCD, _LDCE, _LDCH, _LDCL, _LDCHLM, _LDCA,
    _LDDB, _LDDC, _LDDD, _LDDE, _LDDH, _LDDL, _LDDHLM, _LDDA, _LDEB, _LDEC, _LDED, _LDEE, _LDEH, _LDEL, _LDEHLM, _LDEA,
    _LDHB, _LDHC, _LDHD, _LDHE, _LDHH, _LDHL, _LDHHLM, _LDHA, _LDLB, _LDLC, _LDLD, _LDLE, _LDLH, _LDLL, _LDLHLM, _LDLA,
    _LDMHLB, _LDMHLC, _LDMHLD, _LDMHLE, _LDMHLH, _LDMHLL, HALT, _LDMHLA, _LDAB, _LDAC, _LDAD, _LDAE, _LDAH, _LDAL, _LDAHLM, _LDAA,
    _ADDAB, _ADDAC, _ADDAD, _ADDAE, _ADDAH, _ADDAL, _ADDAHLM, _ADDAA, _ADCAB, _ADCAC, _ADCAD, _ADCAE, _ADCAH, _ADCAL, _ADCAHLM, _ADCAA,
    _SUBB, _SUBC, _SUBD, _SUBE, _SUBH, _SUBL, _SUBMHL, _SUBA, _SBCAB, _SBCAC, _SBCAD, _SBCAE, _SBCAH, _SBCAL, _SBCAHLM, _SBCAA,
    _ANDB, _ANDC, _ANDD, _ANDE, _ANDH, _ANDL, _ANDHLM, _ANDA, _XORB, _XORC, _XORD, _XORE, _XORH, _XORL, _XORHLM, _XORA,
    _ORB, _ORC, _ORD, _ORE, _ORH, _ORL, _ORHLM, _ORA, CPB, _CPC, _CPD, _CPE, _CPH, _CPLR, _CPHLM, _CPA,
    _RETNZ, _POPBC, _JPNZNN, JPNN, _CALLNZNN, _PUSHBC, ADDAN, _RST00H, _RETZ, RET, _JPZNN, _BITSET, _CALLZNN, CALLNN, _ADCAN, _RST08H,
    _RETNC, _POPDE, _JPNCNN, OUTNA, _CALLNCNN, _PUSHDE, SUBN, _RST10H, _RETC, _EXX, _JPCNN, INAN, _CALLCNN, _IXSET, _SBCAN, _RST18H,
    _RETPO, _POPHL, _JPPONN, _EXSPHL, _CALLPONN, _PUSHHL, _ANDN, _RST20H, _RETPE, _JPHLM, _JPPENN, _EXDEHL, _CALLPENN, _MISCSET, _XORN, _RST28H,
    _RETP, _POPAF, _JPPNN, DI, _CALLPNN, _PUSHAF, _ORN, _RST30H, _RETM, _LDSPHL, _JPMNN, EI, _CALLMNN, _IYSET, _CPN, _RST38H, 
}

//TODO: Check address modes
pub const INSTRUCTIONS: [(Instruction, AddressMode); 0x100] = [
    (Instruction::NOP, AddressMode::None), (Instruction::LDBCNN, AddressMode::ImmediateExtended), 
    (Instruction::_LDMBCA, AddressMode::None), (Instruction::_INCBC, AddressMode::None),
    (Instruction::INCB, AddressMode::Register), (Instruction::DECB, AddressMode::Register), 
    (Instruction::LDBN, AddressMode::Immediate), (Instruction::_RLCA, AddressMode::None),
    (Instruction::_EXAFAF, AddressMode::None), (Instruction::_ADDHLBC, AddressMode::None), 
    (Instruction::_LDABCM, AddressMode::None), (Instruction::_DECBC, AddressMode::None),
    (Instruction::_INCC, AddressMode::None), (Instruction::_DECC, AddressMode::None), 
    (Instruction::_LDCN, AddressMode::Immediate), (Instruction::_RRCA, AddressMode::None),
    (Instruction::_DJNZD, AddressMode::Immediate), (Instruction::LDDENN, AddressMode::ImmediateExtended), 
    (Instruction::_LDMDEA, AddressMode::None), (Instruction::_INCDE, AddressMode::None),
    (Instruction::_INCD, AddressMode::None), (Instruction::_DECD, AddressMode::None), 
    (Instruction::_LDDN, AddressMode::Immediate), (Instruction::_RLA, AddressMode::None),
    (Instruction::_JRD, AddressMode::Immediate), (Instruction::_ADDHLDE, AddressMode::None), 
    (Instruction::_LDADEM, AddressMode::None), (Instruction::_DECDE, AddressMode::None),
    (Instruction::_INCE, AddressMode::None), (Instruction::_DECE, AddressMode::None), 
    (Instruction::_LDEN, AddressMode::Immediate), (Instruction::_RRA, AddressMode::None),
    (Instruction::_JRNZD, AddressMode::Immediate), (Instruction::LDHLNNM, AddressMode::ImmediateExtended), 
    (Instruction::LDMNNHL, AddressMode::Extended), (Instruction::INCHL, AddressMode::Register),
    (Instruction::_INCH, AddressMode::None), (Instruction::_DECH, AddressMode::None), 
    (Instruction::_LDHN, AddressMode::Immediate), (Instruction::_DAA, AddressMode::None),
    (Instruction::JRZD, AddressMode::Immediate), (Instruction::_ADDHLHL, AddressMode::None), 
    (Instruction::_LDHLNNM, AddressMode::ImmediateExtended), (Instruction::_DECHL, AddressMode::None),
    (Instruction::_INCL, AddressMode::None), (Instruction::_DECL, AddressMode::None), 
    (Instruction::_LDLN, AddressMode::Immediate), (Instruction::_CPL, AddressMode::None),
    (Instruction::_JRNCD, AddressMode::Immediate), (Instruction::LDSPNN, AddressMode::Extended), 
    (Instruction::_LDMNNA, AddressMode::Extended), (Instruction::_INCSP, AddressMode::None),
    (Instruction::_INCMHL, AddressMode::None), (Instruction::_DECMHL, AddressMode::None), 
    (Instruction::LDMHLN, AddressMode::Immediate), (Instruction::_SCF, AddressMode::None),
    (Instruction::_JRCD, AddressMode::Immediate), (Instruction::_ADDHLSP, AddressMode::None), 
    (Instruction::_LDANNM, AddressMode::Extended), (Instruction::_DECSP, AddressMode::None),
    (Instruction::_INCA, AddressMode::None), (Instruction::_DECA, AddressMode::None), 
    (Instruction::LDAN, AddressMode::Immediate), (Instruction::_CCF, AddressMode::None),
    (Instruction::_LDBB, AddressMode::None), (Instruction::_LDBC, AddressMode::None), 
    (Instruction::_LDBD, AddressMode::None), (Instruction::_LDBE, AddressMode::None),
    (Instruction::_LDBH, AddressMode::None), (Instruction::_LDBL, AddressMode::None), 
    (Instruction::_LDBHLM, AddressMode::None), (Instruction::LDBA, AddressMode::Register),
    (Instruction::_LDCB, AddressMode::None), (Instruction::_LDCC, AddressMode::None), 
    (Instruction::_LDCD, AddressMode::None), (Instruction::_LDCE, AddressMode::None),
    (Instruction::_LDCH, AddressMode::None), (Instruction::_LDCL, AddressMode::None), 
    (Instruction::_LDCHLM, AddressMode::None), (Instruction::_LDCA, AddressMode::None),
    (Instruction::_LDDB, AddressMode::None), (Instruction::_LDDC, AddressMode::None), 
    (Instruction::_LDDD, AddressMode::None), (Instruction::_LDDE, AddressMode::None),
    (Instruction::_LDDH, AddressMode::None), (Instruction::_LDDL, AddressMode::None), 
    (Instruction::_LDDHLM, AddressMode::None), (Instruction::_LDDA, AddressMode::None),
    (Instruction::_LDEB, AddressMode::None), (Instruction::_LDEC, AddressMode::None), 
    (Instruction::_LDED, AddressMode::None), (Instruction::_LDEE, AddressMode::None),
    (Instruction::_LDEH, AddressMode::None), (Instruction::_LDEL, AddressMode::None), 
    (Instruction::_LDEHLM, AddressMode::None), (Instruction::_LDEA, AddressMode::None),
    (Instruction::_LDHB, AddressMode::None), (Instruction::_LDHC, AddressMode::None), 
    (Instruction::_LDHD, AddressMode::None), (Instruction::_LDHE, AddressMode::None),
    (Instruction::_LDHH, AddressMode::None), (Instruction::_LDHL, AddressMode::None), 
    (Instruction::_LDHHLM, AddressMode::None), (Instruction::_LDHA, AddressMode::None),
    (Instruction::_LDLB, AddressMode::None), (Instruction::_LDLC, AddressMode::None), 
    (Instruction::_LDLD, AddressMode::None), (Instruction::_LDLE, AddressMode::None),
    (Instruction::_LDLH, AddressMode::None), (Instruction::_LDLL, AddressMode::None), 
    (Instruction::_LDLHLM, AddressMode::None), (Instruction::_LDLA, AddressMode::None),
    (Instruction::_LDMHLB, AddressMode::None), (Instruction::_LDMHLC, AddressMode::None), 
    (Instruction::_LDMHLD, AddressMode::None), (Instruction::_LDMHLE, AddressMode::None),
    (Instruction::_LDMHLH, AddressMode::None), (Instruction::_LDMHLL, AddressMode::None), 
    (Instruction::HALT, AddressMode::None), (Instruction::_LDMHLA, AddressMode::None),
    (Instruction::_LDAB, AddressMode::None), (Instruction::_LDAC, AddressMode::None), 
    (Instruction::_LDAD, AddressMode::None), (Instruction::_LDAE, AddressMode::None),
    (Instruction::_LDAH, AddressMode::None), (Instruction::_LDAL, AddressMode::None), 
    (Instruction::_LDAHLM, AddressMode::None), (Instruction::_LDAA, AddressMode::None),
    (Instruction::_ADDAB, AddressMode::None), (Instruction::_ADDAC, AddressMode::None), 
    (Instruction::_ADDAD, AddressMode::None), (Instruction::_ADDAE, AddressMode::None),
    (Instruction::_ADDAH, AddressMode::None), (Instruction::_ADDAL, AddressMode::None), 
    (Instruction::_ADDAHLM, AddressMode::None), (Instruction::_ADDAA, AddressMode::None),
    (Instruction::_ADCAB, AddressMode::None), (Instruction::_ADCAC, AddressMode::None), 
    (Instruction::_ADCAD, AddressMode::None), (Instruction::_ADCAE, AddressMode::None),
    (Instruction::_ADCAH, AddressMode::None), (Instruction::_ADCAL, AddressMode::None), 
    (Instruction::_ADCAHLM, AddressMode::None), (Instruction::_ADCAA, AddressMode::None),
    (Instruction::_SUBB, AddressMode::None), (Instruction::_SUBC, AddressMode::None), 
    (Instruction::_SUBD, AddressMode::None), (Instruction::_SUBE, AddressMode::None),
    (Instruction::_SUBH, AddressMode::None), (Instruction::_SUBL, AddressMode::None), 
    (Instruction::_SUBMHL, AddressMode::None), (Instruction::_SUBA, AddressMode::None),
    (Instruction::_SBCAB, AddressMode::None), (Instruction::_SBCAC, AddressMode::None), 
    (Instruction::_SBCAD, AddressMode::None), (Instruction::_SBCAE, AddressMode::None),
    (Instruction::_SBCAH, AddressMode::None), (Instruction::_SBCAL, AddressMode::None), 
    (Instruction::_SBCAHLM, AddressMode::None), (Instruction::_SBCAA, AddressMode::None),
    (Instruction::_ANDB, AddressMode::None), (Instruction::_ANDC, AddressMode::None), 
    (Instruction::_ANDD, AddressMode::None), (Instruction::_ANDE, AddressMode::None),
    (Instruction::_ANDH, AddressMode::None), (Instruction::_ANDL, AddressMode::None), 
    (Instruction::_ANDHLM, AddressMode::None), (Instruction::_ANDA, AddressMode::None),
    (Instruction::_XORB, AddressMode::None), (Instruction::_XORC, AddressMode::None), 
    (Instruction::_XORD, AddressMode::None), (Instruction::_XORE, AddressMode::None),
    (Instruction::_XORH, AddressMode::None), (Instruction::_XORL, AddressMode::None), 
    (Instruction::_XORHLM, AddressMode::None), (Instruction::_XORA, AddressMode::None),
    (Instruction::_ORB, AddressMode::None), (Instruction::_ORC, AddressMode::None), 
    (Instruction::_ORD, AddressMode::None), (Instruction::_ORE, AddressMode::None),
    (Instruction::_ORH, AddressMode::None), (Instruction::_ORL, AddressMode::None), 
    (Instruction::_ORHLM, AddressMode::None), (Instruction::_ORA, AddressMode::None),
    (Instruction::CPB, AddressMode::Register), (Instruction::_CPC, AddressMode::None), 
    (Instruction::_CPD, AddressMode::None), (Instruction::_CPE, AddressMode::None),
    (Instruction::_CPH, AddressMode::None), (Instruction::_CPLR, AddressMode::None), 
    (Instruction::_CPHLM, AddressMode::None), (Instruction::_CPA, AddressMode::None),
    (Instruction::_RETNZ, AddressMode::None), (Instruction::_POPBC, AddressMode::None), 
    (Instruction::_JPNZNN, AddressMode::ImmediateExtended), (Instruction::JPNN, AddressMode::Extended),
    (Instruction::_CALLNZNN, AddressMode::ImmediateExtended), (Instruction::_PUSHBC, AddressMode::None), 
    (Instruction::ADDAN, AddressMode::Immediate), (Instruction::_RST00H, AddressMode::None),
    (Instruction::_RETZ, AddressMode::None), (Instruction::RET, AddressMode::None), 
    (Instruction::_JPZNN, AddressMode::ImmediateExtended), (Instruction::_BITSET, AddressMode::None),
    (Instruction::_CALLZNN, AddressMode::ImmediateExtended), (Instruction::CALLNN, AddressMode::Extended), 
    (Instruction::_ADCAN, AddressMode::Immediate), (Instruction::_RST08H, AddressMode::None),
    (Instruction::_RETNC, AddressMode::None), (Instruction::_POPDE, AddressMode::None), 
    (Instruction::_JPNCNN, AddressMode::ImmediateExtended), (Instruction::OUTNA, AddressMode::Immediate),
    (Instruction::_CALLNCNN, AddressMode::ImmediateExtended), (Instruction::_PUSHDE, AddressMode::None), 
    (Instruction::SUBN, AddressMode::Immediate), (Instruction::_RST10H, AddressMode::None),
    (Instruction::_RETC, AddressMode::None), (Instruction::_EXX, AddressMode::None), 
    (Instruction::_JPCNN, AddressMode::ImmediateExtended), (Instruction::INAN, AddressMode::Immediate),
    (Instruction::_CALLCNN, AddressMode::ImmediateExtended), (Instruction::_IXSET, AddressMode::None), 
    (Instruction::_SBCAN, AddressMode::Immediate), (Instruction::_RST18H, AddressMode::None),
    (Instruction::_RETPO, AddressMode::None), (Instruction::_POPHL, AddressMode::None), 
    (Instruction::_JPPONN, AddressMode::ImmediateExtended), (Instruction::_EXSPHL, AddressMode::None),
    (Instruction::_CALLPONN, AddressMode::ImmediateExtended), (Instruction::_PUSHHL, AddressMode::None), 
    (Instruction::_ANDN, AddressMode::Immediate), (Instruction::_RST20H, AddressMode::None),
    (Instruction::_RETPE, AddressMode::None), (Instruction::_JPHLM, AddressMode::None), 
    (Instruction::_JPPENN, AddressMode::ImmediateExtended), (Instruction::_EXDEHL, AddressMode::None),
    (Instruction::_CALLPENN, AddressMode::ImmediateExtended), (Instruction::_MISCSET, AddressMode::None), 
    (Instruction::_XORN, AddressMode::Immediate), (Instruction::_RST28H, AddressMode::None),
    (Instruction::_RETP, AddressMode::None), (Instruction::_POPAF, AddressMode::None), 
    (Instruction::_JPPNN, AddressMode::ImmediateExtended), (Instruction::DI, AddressMode::None),
    (Instruction::_CALLPNN, AddressMode::ImmediateExtended), (Instruction::_PUSHAF, AddressMode::None), 
    (Instruction::_ORN, AddressMode::Immediate), (Instruction::_RST30H, AddressMode::None),
    (Instruction::_RETM, AddressMode::None), (Instruction::_LDSPHL, AddressMode::None), 
    (Instruction::_JPMNN, AddressMode::ImmediateExtended), (Instruction::EI, AddressMode::None),
    (Instruction::_CALLMNN, AddressMode::ImmediateExtended), (Instruction::_IYSET, AddressMode::None), 
    (Instruction::_CPN, AddressMode::Immediate), (Instruction::_RST38H, AddressMode::None),
];