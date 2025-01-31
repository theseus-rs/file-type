use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860063: FileFormat = FileFormat {
    id: 105_860_063,
    puid: "wikidata/105860063",
    name: "ParaView VTK Image data",
    extensions: &["vti"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
