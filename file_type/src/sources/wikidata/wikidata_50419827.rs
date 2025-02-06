use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50419827: FileFormat = FileFormat {
    id: 50_419_827,
    source_type: SourceType::Wikidata,
    name: "Microsoft PRX File",
    extensions: &["prx"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
