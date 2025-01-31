use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967105: FileFormat = FileFormat {
    id: 27_967_105,
    puid: "wikidata/27967105",
    name: "SHO",
    extensions: &["sho"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
