use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_731135: FileFormat = FileFormat {
    id: 731_135,
    source_type: SourceType::Wikidata,
    name: "Microsoft Management Console",
    extensions: &["msc"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
