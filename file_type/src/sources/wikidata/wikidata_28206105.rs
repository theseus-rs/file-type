use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206105: FileFormat = FileFormat {
    id: 28_206_105,
    puid: "wikidata/28206105",
    name: "Falcon True Color",
    extensions: &["ftc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
