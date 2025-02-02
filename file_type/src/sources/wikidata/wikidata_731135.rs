use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_731135: FileFormat = FileFormat {
    id: 731_135,
    source_type: SourceType::Wikidata,
    name: "Microsoft Management Console",
    extensions: &["msc"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
