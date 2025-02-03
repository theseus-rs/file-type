use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_77433095: FileFormat = FileFormat {
    id: 77_433_095,
    source_type: SourceType::Wikidata,
    name: "YaST MetaPackage",
    extensions: &["ymp"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
