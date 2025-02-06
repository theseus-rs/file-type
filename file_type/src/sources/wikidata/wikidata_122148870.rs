use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122148870: FileFormat = FileFormat {
    id: 122_148_870,
    source_type: SourceType::Wikidata,
    name: "My Logo Maker File",
    extensions: &["myf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
