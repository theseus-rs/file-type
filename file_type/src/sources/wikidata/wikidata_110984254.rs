use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110984254: FileFormat = FileFormat {
    id: 110_984_254,
    puid: "wikidata/110984254",
    name: "Corel VideoStudio Project File",
    extensions: &["vsp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
