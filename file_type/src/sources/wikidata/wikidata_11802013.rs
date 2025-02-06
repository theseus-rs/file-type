use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_11802013: FileFormat = FileFormat {
    id: 11_802_013,
    source_type: SourceType::Wikidata,
    name: "Sega Dreamcast Texture Package Format",
    extensions: &["pvm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
