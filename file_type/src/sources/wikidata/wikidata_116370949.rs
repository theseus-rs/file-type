use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116370949: FileFormat = FileFormat {
    id: 116_370_949,
    source_type: SourceType::Wikidata,
    name: "DFPWM",
    extensions: &["dfpwm"],
    media_types: &["audio/dfpwm"],
    signatures: &[],
    related_formats: &[],
};
