use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857392: FileFormat = FileFormat {
    id: 105_857_392,
    source_type: SourceType::Wikidata,
    name: "Java Web Start application descriptor",
    extensions: &["jnlp"],
    media_types: &["text.xnk"],
    internal_signatures: &[],
    related_formats: &[],
};
