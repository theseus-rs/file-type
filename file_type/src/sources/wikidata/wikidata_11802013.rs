use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_11802013: FileFormat = FileFormat {
    id: 11_802_013,
    source_type: SourceType::Wikidata,
    name: "Sega Dreamcast Texture Package Format",
    extensions: &["pvm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
