use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_77433095: FileFormat = FileFormat {
    id: 77_433_095,
    source_type: SourceType::Wikidata,
    name: "YaST MetaPackage",
    extensions: &["ymp"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
