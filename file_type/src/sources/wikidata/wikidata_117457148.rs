use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117457148: FileFormat = FileFormat {
    id: 117_457_148,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Encrypted Database File 2.0",
    extensions: &["mda", "mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
