use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_66303013: FileFormat = FileFormat {
    id: 66_303_013,
    source_type: SourceType::Wikidata,
    name: "Lotus 1-2-3 Educational Version Worksheet file",
    extensions: &["wke"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
