use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116370949: FileFormat = FileFormat {
    id: 116_370_949,
    source_type: SourceType::Wikidata,
    name: "DFPWM",
    extensions: &["dfpwm"],
    media_types: &["audio/dfpwm"],
    internal_signatures: &[],
    related_formats: &[],
};
