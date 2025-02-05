use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_131293815: FileFormat = FileFormat {
    id: 131_293_815,
    source_type: SourceType::Wikidata,
    name: "Tera Term macro source code file",
    extensions: &["ttl"],
    media_types: &["text/x-teratermmacro"],
    signatures: &[],
    related_formats: &[],
};
