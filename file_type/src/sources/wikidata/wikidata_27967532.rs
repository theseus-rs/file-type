use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967532: FileFormat = FileFormat {
    id: 27_967_532,
    source_type: SourceType::Wikidata,
    name: "DVD Information File",
    extensions: &["bup", "ifo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
