use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_127803507: FileFormat = FileFormat {
    id: 127_803_507,
    source_type: SourceType::Wikidata,
    name: "Mojo source code file",
    extensions: &["mojo", "🔥"],
    media_types: &["application/x-mojo", "text/x-mojo"],
    signatures: &[],
    related_formats: &[],
};
