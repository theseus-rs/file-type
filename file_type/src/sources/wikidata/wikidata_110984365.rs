use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110984365: FileFormat = FileFormat {
    id: 110_984_365,
    puid: "wikidata/110984365",
    name: "Corel VideoStudio HTML5 Project File",
    extensions: &["vsh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
