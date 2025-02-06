use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205372: FileFormat = FileFormat {
    id: 28_205_372,
    source_type: SourceType::Wikidata,
    name: "Kodak TIFF",
    extensions: &["tif"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
