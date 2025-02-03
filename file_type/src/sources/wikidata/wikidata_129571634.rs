use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_129571634: FileFormat = FileFormat {
    id: 129_571_634,
    source_type: SourceType::Wikidata,
    name: "Hy source code file",
    extensions: &["hy"],
    media_types: &["application/x-hy", "text/x-hy"],
    internal_signatures: &[],
    related_formats: &[],
};
