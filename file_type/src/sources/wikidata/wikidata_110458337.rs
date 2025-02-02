use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_110458337: FileFormat = FileFormat {
    id: 110_458_337,
    source_type: SourceType::Wikidata,
    name: "SpritePad Image Format",
    extensions: &["spd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
