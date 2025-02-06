use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_71301157: FileFormat = FileFormat {
    id: 71_301_157,
    source_type: SourceType::Wikidata,
    name: "WHIP! DWF Format",
    extensions: &["dwf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
