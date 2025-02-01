use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116370949: FileFormat = FileFormat {
    id: 116_370_949,
    puid: "wikidata/116370949",
    name: "DFPWM",
    extensions: &["dfpwm"],
    media_types: &["audio/dfpwm"],
    internal_signatures: &[],
    related_formats: &[],
};
