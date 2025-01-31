use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205727: FileFormat = FileFormat {
    id: 28_205_727,
    puid: "wikidata/28205727",
    name: "AVS X image",
    extensions: &["avs", "mbfavs", "x"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
