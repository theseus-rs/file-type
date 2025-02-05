use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117844169: FileFormat = FileFormat {
    id: 117_844_169,
    source_type: SourceType::Wikidata,
    name: "Kofax Group 4 file",
    extensions: &["kfx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
