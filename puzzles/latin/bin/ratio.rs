
const TIMEOUT: std::time::Duration = std::time::Duration::from_mins(5);

fn main() 
{
    type Audit<Model> = latin::method::binary::audit::Audit<Model>;

    type Field<Model> = arc::assert::field::FieldV5<Model, bitset::flat::BitSet>;

    type Queue<Model> = arc::assert::queue::QueueV3<Model, bitset::meta::BitSet>;

    type Cache<Model> = arc::assert::cache::CacheV4<Model, bitset::flat::BitSet>;

    type Probe<Model> = arc::coerce::probe::ProbeV3<Model>;

    for size in 4..=30
    {
        let model = & latin::method::binary::model::ScalarModel::new(size);

        let Some(report) = arc::analyze::gauge::<_, Audit<_>, Field<_>, Queue<_>, Cache<_>, Probe<_>>(model, TIMEOUT) else
        {
            println!("{:} skip", size);

            continue;
        };

        let ratio = report.locates as f64 / report.coerces as f64;
        
        println!("{:} {:?} {:.0?}", size, report, ratio);
    };
}

// 4 GaugeReport { locates: 606, settles: 568, negates: 38, selects: 8, coerces: 7, reverts: 0, elapsed: 39.7µs } 87
// 5 GaugeReport { locates: 1665, settles: 1585, negates: 80, selects: 14, coerces: 13, reverts: 0, elapsed: 70.7µs } 128
// 6 GaugeReport { locates: 3767, settles: 3625, negates: 142, selects: 25, coerces: 24, reverts: 0, elapsed: 168.5µs } 157
// 7 GaugeReport { locates: 7404, settles: 7170, negates: 234, selects: 36, coerces: 35, reverts: 0, elapsed: 341.2µs } 212
// 8 GaugeReport { locates: 13237, settles: 12880, negates: 357, selects: 52, coerces: 51, reverts: 0, elapsed: 668.5µs } 260
// 9 GaugeReport { locates: 22234, settles: 21715, negates: 519, selects: 74, coerces: 73, reverts: 1, elapsed: 1.2954ms } 305
// 10 GaugeReport { locates: 34398, settles: 33682, negates: 716, selects: 101, coerces: 100, reverts: 1, elapsed: 993.1µs } 344
// 11 GaugeReport { locates: 51971, settles: 50991, negates: 980, selects: 134, coerces: 133, reverts: 4, elapsed: 1.4564ms } 391
// 12 GaugeReport { locates: 83363, settles: 81664, negates: 1699, selects: 197, coerces: 196, reverts: 40, elapsed: 2.3622ms } 425
// 13 GaugeReport { locates: 104982, settles: 103339, negates: 1643, selects: 198, coerces: 197, reverts: 5, elapsed: 2.9507ms } 533
// 14 GaugeReport { locates: 151699, settles: 149316, negates: 2383, selects: 273, coerces: 272, reverts: 35, elapsed: 4.5216ms } 558
// 15 GaugeReport { locates: 194433, settles: 191787, negates: 2646, selects: 297, coerces: 296, reverts: 14, elapsed: 5.6103ms } 657
// 17 GaugeReport { locates: 351214, settles: 346316, negates: 4898, selects: 492, coerces: 491, reverts: 93, elapsed: 11.2015ms } 715
// 18 GaugeReport { locates: 419665, settles: 414689, negates: 4976, selects: 504, coerces: 503, reverts: 44, elapsed: 14.2415ms } 834
// 19 GaugeReport { locates: 521442, settles: 515803, negates: 5639, selects: 571, coerces: 570, reverts: 37, elapsed: 18.3481ms } 915
// 20 GaugeReport { locates: 1000662, settles: 985359, negates: 15303, selects: 1251, coerces: 1250, reverts: 645, elapsed: 45.4271ms } 801
// 21 GaugeReport { locates: 827923, settles: 819264, negates: 8659, selects: 839, coerces: 838, reverts: 148, elapsed: 35.7346ms } 988
// 22 GaugeReport { locates: 993196, settles: 983815, negates: 9381, selects: 873, coerces: 872, reverts: 98, elapsed: 44.1532ms } 1139
// 23 GaugeReport { locates: 1250355, settles: 1237450, negates: 12905, selects: 1088, coerces: 1087, reverts: 227, elapsed: 64.2431ms } 1150
// 24 GaugeReport { locates: 1386626, settles: 1375318, negates: 11308, selects: 1037, coerces: 1036, reverts: 61, elapsed: 70.6678ms } 1338
// 25 GaugeReport { locates: 1597414, settles: 1585540, negates: 11874, selects: 1086, coerces: 1085, reverts: 7, elapsed: 86.2463ms } 1472
// 26 GaugeReport { locates: 2120419, settles: 2101507, negates: 18912, selects: 1542, coerces: 1541, reverts: 354, elapsed: 143.3546ms } 1376
// 27 GaugeReport { locates: 2400718, settles: 2382282, negates: 18436, selects: 1537, coerces: 1536, reverts: 208, elapsed: 162.8351ms } 1563
// 28 GaugeReport { locates: 3864161, settles: 3811642, negates: 52519, selects: 3890, coerces: 3889, reverts: 2448, elapsed: 109.3597ms } 994
// 29 GaugeReport { locates: 4279104, settles: 4230835, negates: 48269, selects: 3442, coerces: 3441, reverts: 1866, elapsed: 114.8676ms } 1244
// 30 GaugeReport { locates: 3420579, settles: 3399118, negates: 21461, selects: 1775, coerces: 1774, reverts: 63, elapsed: 84.6228ms } 1928