use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28919071: FileFormat = FileFormat {
    id: 28_919_071,
    puid: "wikidata/28919071",
    name: "After Effects project, binary variant",
    extensions: &["aep"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
