use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129571634: FileFormat = FileFormat {
    id: 129_571_634,
    source_type: SourceType::Wikidata,
    name: "Hy source code file",
    extensions: &["hy"],
    media_types: &["application/x-hy", "text/x-hy"],
    signatures: &[],
    related_formats: &[],
};
