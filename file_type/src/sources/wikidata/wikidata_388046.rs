use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_388046: FileFormat = FileFormat {
    id: 388_046,
    source_type: SourceType::Wikidata,
    name: "X-Face",
    extensions: &["face", "xface"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
