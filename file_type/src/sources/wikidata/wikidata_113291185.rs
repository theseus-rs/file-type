use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113291185: FileFormat = FileFormat {
    id: 113_291_185,
    source_type: SourceType::Wikidata,
    name: "Serif Metafile Format",
    extensions: &["smf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
