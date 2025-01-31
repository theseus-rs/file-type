use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111263105: FileFormat = FileFormat {
    id: 111_263_105,
    puid: "wikidata/111263105",
    name: "Typhoon wave file",
    extensions: &["c01"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
