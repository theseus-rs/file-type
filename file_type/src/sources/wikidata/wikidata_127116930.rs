use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127116930: FileFormat = FileFormat {
    id: 127_116_930,
    source_type: SourceType::Wikidata,
    name: "IDLSAV file",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
