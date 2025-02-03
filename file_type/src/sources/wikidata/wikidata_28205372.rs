use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28205372: FileFormat = FileFormat {
    id: 28_205_372,
    source_type: SourceType::Wikidata,
    name: "Kodak TIFF",
    extensions: &["tif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
