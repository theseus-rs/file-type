use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_63166396: FileFormat = FileFormat {
    id: 63_166_396,
    source_type: SourceType::Wikidata,
    name: "Microsoft Works Database for Macintosh, version 3",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    signatures: &[],
    related_formats: &[],
};
