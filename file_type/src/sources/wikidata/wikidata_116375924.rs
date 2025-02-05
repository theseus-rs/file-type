use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116375924: FileFormat = FileFormat {
    id: 116_375_924,
    source_type: SourceType::Wikidata,
    name: "Access Database (2003 and earlier)",
    extensions: &["mdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
