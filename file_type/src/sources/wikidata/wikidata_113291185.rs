use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113291185: FileFormat = FileFormat {
    id: 113_291_185,
    source_type: SourceType::Wikidata,
    name: "Serif Metafile Format",
    extensions: &["smf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
