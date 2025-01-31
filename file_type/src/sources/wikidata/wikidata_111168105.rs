use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111168105: FileFormat = FileFormat {
    id: 111_168_105,
    puid: "wikidata/111168105",
    name: "ChemSketch 2.0 Document",
    extensions: &["sk2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
