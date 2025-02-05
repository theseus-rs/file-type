use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205974: FileFormat = FileFormat {
    id: 28_205_974,
    source_type: SourceType::Wikidata,
    name: "Digital Video Interactive Y Luminance Channel",
    extensions: &["imy"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
