use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122148870: FileFormat = FileFormat {
    id: 122_148_870,
    source_type: SourceType::Wikidata,
    name: "My Logo Maker File",
    extensions: &["myf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
