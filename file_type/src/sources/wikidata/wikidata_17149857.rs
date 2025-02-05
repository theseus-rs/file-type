use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_17149857: FileFormat = FileFormat {
    id: 17_149_857,
    source_type: SourceType::Wikidata,
    name: "zone file",
    extensions: &["zone"],
    media_types: &["text/dns"],
    signatures: &[],
    related_formats: &[],
};
