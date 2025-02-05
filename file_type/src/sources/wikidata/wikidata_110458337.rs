use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110458337: FileFormat = FileFormat {
    id: 110_458_337,
    source_type: SourceType::Wikidata,
    name: "SpritePad Image Format",
    extensions: &["spd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
