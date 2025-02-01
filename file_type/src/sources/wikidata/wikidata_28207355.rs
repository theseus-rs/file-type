use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207355: FileFormat = FileFormat {
    id: 28_207_355,
    puid: "wikidata/28207355",
    name: "TrueVista",
    extensions: &["vst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
