use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757978: FileFormat = FileFormat {
    id: 28_757_978,
    source_type: SourceType::Wikidata,
    name: "Precompiled Windows Setup Information File",
    extensions: &["pnf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
