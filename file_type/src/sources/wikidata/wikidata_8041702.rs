use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_8041702: FileFormat = FileFormat {
    id: 8_041_702,
    source_type: SourceType::Wikidata,
    name: "eXtended Binary",
    extensions: &["xb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
