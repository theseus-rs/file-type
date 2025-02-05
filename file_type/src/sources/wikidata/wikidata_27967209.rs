use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967209: FileFormat = FileFormat {
    id: 27_967_209,
    source_type: SourceType::Wikidata,
    name: "Pro Tracker v3.xx module",
    extensions: &["pt3"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
