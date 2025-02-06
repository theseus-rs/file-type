use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120750742: FileFormat = FileFormat {
    id: 120_750_742,
    source_type: SourceType::Wikidata,
    name: "OpenRP",
    extensions: &["rp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
