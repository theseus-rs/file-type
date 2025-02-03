use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_127803507: FileFormat = FileFormat {
    id: 127_803_507,
    source_type: SourceType::Wikidata,
    name: "Mojo source code file",
    extensions: &["mojo", "🔥"],
    media_types: &["application/x-mojo", "text/x-mojo"],
    internal_signatures: &[],
    related_formats: &[],
};
