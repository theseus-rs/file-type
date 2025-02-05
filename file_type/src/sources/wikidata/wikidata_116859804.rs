use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116859804: FileFormat = FileFormat {
    id: 116_859_804,
    source_type: SourceType::Wikidata,
    name: "Peachtree Vendor List",
    extensions: &["csv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
