use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979154: FileFormat = FileFormat {
    id: 27_979_154,
    puid: "wikidata/27979154",
    name: "ArtWorx Data Format",
    extensions: &["adf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
