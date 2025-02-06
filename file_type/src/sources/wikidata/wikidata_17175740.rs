use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17175740: FileFormat = FileFormat {
    id: 17_175_740,
    source_type: SourceType::Wikidata,
    name: "comic book archive, tar container",
    extensions: &["cbt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
