use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116375924: FileFormat = FileFormat {
    id: 116_375_924,
    source_type: SourceType::Wikidata,
    name: "Access Database (2003 and earlier)",
    extensions: &["mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
