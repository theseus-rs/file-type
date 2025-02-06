use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826386: FileFormat = FileFormat {
    id: 27_826_386,
    source_type: SourceType::Wikidata,
    name: "OVR pyramid file",
    extensions: &["ovr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
