use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63177205: FileFormat = FileFormat {
    id: 63_177_205,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Macintosh, version 4",
    extensions: &["wdb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
